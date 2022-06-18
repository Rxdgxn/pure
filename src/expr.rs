const EXTRA: [&str; 1] = ["setColor"];

pub fn write(l: &str, fc: &mut String) {
    if !l.is_empty() {
        if l.chars().nth(0).unwrap() == '@' {
            fc.push_str(&expr(&l.trim()[1..l.trim().len()]));
        }
        else {
            fc.push_str(l);
        }
    }
    fc.push('\n');
}

pub fn expr(src: &str) -> String {
    let mut cw = String::new();
    let mut ret = String::new();
    let mut in_string = false;
    for (i, ch) in src.chars().enumerate() {
        if i != src.len() - 1 {
            if in_string {
                if ch == '\"' {
                    in_string = false;
                }
                cw.push(ch);
            }
            else {
                if ch == '\"' {
                    in_string = true;
                    cw.push(ch);
                }
                else {
                    if ch == ' ' {
                        let cw = &cw as &str;
                        ret.push_str(match cw {
                            "print"    => "love.graphics.print(",
                            "lineRect" => "love.graphics.rectangle(\'line\', ",
                            "fillRect" => "love.graphics.rectangle(\'fill\', ",
                            "setColor" => "love.graphics.setColor(love.math.colorFromBytes(",
                            "draw"     => "love.graphics.draw(",
                            _          => ""
                        });
                        let s = src.trim().split(' ');
                        let mut ci = 0;
                        for xs in s {
                            if ci != 0 && !xs.is_empty() {
                                ret.push_str(xs);
                                ret.push_str(", ");
                            }
                            ci += 1;
                        }
                        ret.push_str("nil)");
                        if EXTRA.contains(&cw) {
                            ret.push_str(")");
                            // Maybe add Hashmaps for multiple brackets?
                        }
                        break;
                    }
                }
                cw.push(ch);
            }
        }
    }

    ret
}