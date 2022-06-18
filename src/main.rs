mod expr;
use expr::*;

fn find_idx(src: &String, k: &str) -> Option<usize> {
    let mut i = 0usize;
    for l in src.lines() {
        if l.trim() == k {
            return Some(i);
        }
        i += 1;
    }
    None
}
fn find_dot(src: &String) -> usize {
    let mut i = 0usize;
    let mut idx = 0usize;
    for ch in src.chars() {
        if ch == '.' {
            idx = i;
        }
        i += 1;
    }
    idx
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_name = &args[1];
    let f = std::fs::read_to_string(file_name).expect("Failed");
    let mut idx = 0;
    let mut segms: Vec<usize> = Vec::new();
    let mut file_content = String::new();

    for segm in ["LOAD:", "UPDATE:", "DRAW:", ":END"] {
        if let Some(x) = find_idx(&f, segm) {
            segms.push(x);
        }
        else {
            panic!("Expected '{}' segment", segm);
        }
    }

    for l in f.lines() {
        let l = l.trim();
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
                write(l, &mut file_content);
            }
            else {
                if idx == segms[1] {
                    file_content.push_str("end\n");
                    file_content.push_str("function love.update()\n")
                }
                else {
                    if idx < segms[2] {
                        write(l, &mut file_content);
                    }
                    else {
                        if idx == segms[2] {
                            file_content.push_str("end\n");
                            file_content.push_str("function love.draw()\n")
                        }
                        else {
                            if idx < segms[3] {
                                write(l, &mut file_content);
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
    let mut path = String::from(&(&file_name as &str)[0..find_dot(&file_name)]);
    path.push_str(".lua");
    std::fs::write(path, file_content).expect("Failed");
}
