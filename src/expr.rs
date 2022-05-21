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
                            ret.push_str("print(");
                            let mut p = false;
                            for c in src[i+1..src.len()].chars() {
                                if c == ' ' && !p {
                                    ret.push(',');
                                    p = true;
                                }
                                else if c != ' ' {
                                    p = false;
                                }
                                ret.push(c);
                            }
                            ret.push_str(")\n");
                        }
                        else if cw == "fillRect".to_string() {
                            ret.push_str("love.graphics.rectangle(\'fill\', ");
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
                        }
                        else if cw == "lineRect".to_string() {
                            ret.push_str("love.graphics.rectangle(\'line\', ");
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
                        }
                        else if cw == "setColor".to_string() {
                            ret.push_str("love.graphics.setColor(love.math.colorFromBytes(");
                            let s = src.trim().split(' ');
                            let mut ci = 0;
                            for xs in s {
                                if ci != 0 && !xs.is_empty() {
                                    ret.push_str(xs);
                                    ret.push_str(", ");
                                }
                                ci += 1;
                            }
                            ret.push_str("nil))");
                        }
                        else if cw == "draw".to_string() {
                            ret.push_str("love.graphics.draw(");
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
                        }
                    }
                }
                cw.push(ch);
            }
        }
    }

    ret
}