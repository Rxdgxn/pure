
pub fn args(src: &str, ret: &mut String) {
    let s = src.trim().split(' ');
    let mut ci = 0;
    for xs in s {
        if ci != 0 && !xs.is_empty() {
            ret.push_str(xs);
            ret.push_str(", ");
        }
        ci += 1;
    }
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
                        if cw == "print".to_string() {
                            ret.push_str("love.graphics.print(");
                            args(src, &mut ret);
                            ret.push_str("nil)");
                        }
                        else if cw == "fillRect".to_string() {
                            ret.push_str("love.graphics.rectangle(\'fill\', ");
                            args(src, &mut ret);
                            ret.push_str("nil)");
                        }
                        else if cw == "lineRect".to_string() {
                            ret.push_str("love.graphics.rectangle(\'line\', ");
                            args(src, &mut ret);
                            ret.push_str("nil)");
                        }
                        else if cw == "setColor".to_string() {
                            ret.push_str("love.graphics.setColor(love.math.colorFromBytes(");
                            args(src, &mut ret);
                            ret.push_str("nil))");
                        }
                        else if cw == "draw".to_string() {
                            ret.push_str("love.graphics.draw(");
                            args(src, &mut ret);
                            ret.push_str("nil)");
                        }
                    }
                }
                cw.push(ch);
            }
        }
    }

    ret
}