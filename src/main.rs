mod expr;
use expr::*;

fn loadidx(src: &String) -> Option<u32> {
    let mut i = 0;
    for l in src.lines() {
        if l.trim() == "LOAD:" {
            return Some(i);
        }
        i += 1;
    }
    None
}
fn updateidx(src: &String) -> Option<u32> {
    let mut i = 0;
    for l in src.lines() {
        if l.trim() == "UPDATE:" {
            return Some(i);
        }
        i += 1;
    }
    None
}
fn drawidx(src: &String) -> Option<u32> {
    let mut i = 0;
    for l in src.lines() {
        if l.trim() == "DRAW:" {
            return Some(i);
        }
        i += 1;
    }
    None
}
fn endidx(src: &String) -> Option<u32> {
    let mut i = 0;
    for l in src.lines() {
        if l.trim() == ":END" {
            return Some(i);
        }
        i += 1;
    }
    None
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let f = std::fs::read_to_string(&args[1]).expect("Failed");
    let mut idx = 0;
    let mut segms: Vec<u32> = Vec::new();
    let mut file_content = String::new();

    if let Some(l) = loadidx(&f) {
        segms.push(l);
    }
    else {
        println!("Expected 'LOAD:' segment");
        std::process::exit(1);
    }
    if let Some(u) = updateidx(&f) {
        segms.push(u);
    }
    else {
        println!("Expected 'UPDATE:' segment");
        std::process::exit(1);
    }
    if let Some(d) = drawidx(&f) {
        segms.push(d);
    }
    else {
        println!("Expected 'DRAW:' segment");
        std::process::exit(1);
    }
    if let Some(e) = endidx(&f) {
        segms.push(e);
    }
    else {
        println!("Expected ':END' segment");
        std::process::exit(1);
    }

    for l in f.lines() {
        if idx < segms[0] {
            if l.len() > 4 {
                if &l[0..3] == "obj" && &l[3..=3] == " " {
                    let s = l[4..l.trim().len()].split(' ');
                    for w in s {
                        file_content.push_str(w);
                        file_content.push_str("={}\n");
                    }
                }
            }
        }
        else {
            if idx == segms[0] {
                file_content.push_str("function love.load()\n");
            }
            else if idx < segms[1] {
                if !l.is_empty() {
                    if l.chars().nth(0).unwrap() == '@' {
                        file_content.push_str(&expr(&l.trim()[1..l.trim().len()]));
                    }
                    else {
                        file_content.push_str(l);
                    }
                }
                file_content.push('\n');
            }
            else {
                if idx == segms[1] {
                    file_content.push_str("end\n");
                    file_content.push_str("function love.update()\n")
                }
                else {
                    if idx < segms[2] {
                        if !l.is_empty() {
                            if l.chars().nth(0).unwrap() == '@' {
                                file_content.push_str(&expr(&l.trim()[1..l.trim().len()]));
                            }
                            else {
                                file_content.push_str(l);
                            }
                        }
                        file_content.push('\n');
                    }
                    else {
                        if idx == segms[2] {
                            file_content.push_str("end\n");
                            file_content.push_str("function love.draw()\n")
                        }
                        else {
                            if idx < segms[3] {
                                if !l.is_empty() {
                                    if l.chars().nth(0).unwrap() == '@' {
                                        file_content.push_str(&expr(&l.trim()[1..l.trim().len()]));
                                    }
                                    else {
                                        file_content.push_str(l);
                                    }
                                }
                                file_content.push('\n');
                            }
                            else {
                                file_content.push_str("end");
                                break;
                            }
                        }
                    }
                }
            }
        }
        idx += 1;
    }
    println!("{file_content}");
}
