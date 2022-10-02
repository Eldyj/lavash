pub fn compile(what: String) -> String {
    let content = what;
    let to_change = content.split("\n");

    let mut functions: Vec<&str> = Vec::new();
    let mut variables: Vec<&str> = Vec::new();

    let mut nesting_level: u8 = 0;
    let mut is_object: bool = false;
    let mut end_close_func: bool = false;
    let mut commented: bool = false;

    let mut is_str_1 = false;
    let mut is_str_2 = false;

    let mut preprocessed_content: String = "".to_string();

    for line in to_change {
        if line != "" {
            let mut curline: String = "".to_string();
            let mut hasfun: bool = false;
            let mut sfi: bool = false; // sfi = skip first indent
            let mut newline: bool = false;

            let mut isdefstart: bool = false;
            let mut isdef: bool = false;
            let mut getdefname: bool = false;

            let mut skip_comma: bool = false;
            let mut isconst: bool = false;
            let mut semicolon: bool = true;
            let mut lastword: &str = "";

            for word in line.split(" ") {
                if word != "" {
                    let end_start: [char; 2];
                    match word.len() {
                        1 => {
                            end_start = [word.chars().nth(0).unwrap(), ' '];
                        }

                        _ => {
                            end_start = [
                                word.chars().nth(0).unwrap(),
                                word.chars().nth(word.len() - 1).unwrap(),
                            ];
                        }
                    }

                    match end_start {
                        ['\'', '\''] | ['"', '"'] => {
                            print!("");
                        }

                        ['\'', _] | [_, '\''] => {
                            if !is_str_2 {
                                is_str_1 = !is_str_1;
                            }
                            curline.push_str(format!("{} ", word).as_str());
                            continue;
                        }

                        ['"', _] | [_, '"'] => {
                            if !is_str_1 {
                                is_str_2 = !is_str_2;
                            }
                            curline.push_str(format!("{} ", word).as_str());
                            continue;
                        }

                        _ => {
                            print!("");
                        }
                    }

                    if !is_str_1 && !is_str_2 {
                        match word {
                            "--" => {
                                commented = true;
                            }

                            "=#" => {
                                curline.push_str("!= ");
                            }

                            "function" => {
                                isdefstart = true;
                                getdefname = true;
                                nesting_level += 1;
                                sfi = true;
                                semicolon = false;
                            }

                            "then" | "do" => {
                                curline.push_str("{");
                                nesting_level += 1;
                                sfi = true;
                                semicolon = false;
                            }

                            "end" | "done" => {
                                curline.push_str("}");
                                nesting_level -= 1;
                                semicolon = false;
                                if is_object {
                                    is_object = false;
                                    if end_close_func {
                                        end_close_func = false;
                                        hasfun = true;
                                    }
                                }

                                if nesting_level == 0 {
                                    newline = true;
                                }
                            }

                            "if" => {
                                curline.push_str("if ");
                                semicolon = false
                            }

                            "elseif" => {
                                curline.push_str("} else if ");
                                nesting_level -= 1;
                            }

                            "else" => {
                                curline.push_str("} else {");
                                sfi = true;
                                semicolon = false;
                            }

                            "abs"
                            | "acos"
                            | "acosh"
                            | "add"
                            | "append"
                            | "asin"
                            | "asinh"
                            | "assert"
                            | "atan"
                            | "atan2"
                            | "atanh"
                            | "b64decode"
                            | "b64encode"
                            | "cbrt"
                            | "ceil"
                            | "cos"
                            | "cosh"
                            | "display"
                            | "edit_obj"
                            | "exp"
                            | "exp2"
                            | "exp_m1"
                            | "extend_trigger_func"
                            | "floor"
                            | "fract"
                            | "get_input"
                            | "http_request"
                            | "hypot"
                            | "ln"
                            | "log"
                            | "matches"
                            | "max"
                            | "min"
                            | "mutability"
                            | "pop"
                            | "print"
                            | "random"
                            | "redfile"
                            | "regex"
                            | "remove_index"
                            | "round"
                            | "sin"
                            | "sinh"
                            | "split_str"
                            | "spwn_version"
                            | "sqrt"
                            | "substr"
                            | "tan"
                            | "tanh"
                            | "time"
                            | "trigger_fn_context"
                            | "writefile" => {
                                curline.push_str(format!("$.{}(", word.to_string()).as_str());
                                hasfun = true;
                            }

                            "obj" => {
                                curline.push_str("obj{");
                                is_object = true;
                                nesting_level += 1;
                                skip_comma = true;
                                sfi = true;
                                if hasfun {
                                    hasfun = false;
                                    end_close_func = true;
                                }
                            }

                            "const" => {
                                isconst = true;
                            }

                            "=" => {
                                if isconst {
                                    curline.push_str("= ");
                                } else if is_object {
                                    curline.push_str(": ");
                                } else {
                                    let mut used_var = false;

                                    for (_, variable) in variables.iter().enumerate() {
                                        if &lastword == variable {
                                            used_var = true;
                                        }
                                    }

                                    if !used_var {
                                        variables.push(lastword);
                                        curline = "let ".to_string();
                                        curline.push_str(lastword);
                                        curline.push_str(" = ");
                                    } else {
                                        curline.push_str("= ")
                                    }
                                }
                            }

                            _ => {
                                if word != "" && !commented {
                                    curline.push_str(word);
                                    curline.push_str(" ");
                                    if getdefname {
                                        functions.push(word);
                                    } else {
                                        for (_, def) in functions.iter().enumerate() {
                                            if def == &word {
                                                curline.replace_range(
                                                    curline.len() - 1..curline.len(),
                                                    "",
                                                );
                                                curline.push_str("(");
                                                hasfun = true;
                                            }
                                        }
                                    }

                                    if isdefstart {
                                        curline.push_str("= (");
                                        isdefstart = false;
                                        isdef = true;
                                    }
                                }
                            }
                        }
                    } else {
                        curline.push_str(format!("{word} ").as_str());
                    }

                    if commented {
                        continue;
                    }
                }

                lastword = word;
            }

            commented = false;
            if curline != "" {
                if curline.chars().nth(curline.len() - 1).unwrap() == ' ' {
                    curline.replace_range(curline.len() - 1..curline.len(), "");
                }

                if hasfun {
                    curline.push_str(")");
                } else if isdef {
                    curline.push_str("){");
                }

                if is_object {
                    if !skip_comma {
                        curline.push(',');
                    }
                } else if semicolon {
                    curline.push(';');
                }

                for i in 0..nesting_level {
                    if i == 0 && sfi {
                    } else {
                        preprocessed_content.push_str("  ");
                    }
                }

                preprocessed_content.push_str(&(format!("{curline}\n").as_str()));

                if newline {
                    preprocessed_content.push_str("\n");
                }
            }
        }
    }
    return preprocessed_content;
}
