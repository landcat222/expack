use std::collections::HashMap;
use crate::read_src::Src;

fn src_to_cmds(src: Src) -> HashMap<String,Src> {
    let mut cmds: HashMap<String,Src> = HashMap::new();

    let mut key = String::new();
    let mut val = Vec::new();
    let mut fn_flag = false;
    for lines in src {
        let mut tmp = Vec::new();
        for cmd in lines {
            if key == String::new() {
                key = cmd.clone();
            } else if cmd == "{".to_string() {
                fn_flag = true;
            } else if cmd == "}".to_string() {
                fn_flag = false;
            } else {
                tmp.push(cmd);
            }
        }
        if tmp.len() > 0 {
            val.push(tmp.clone());
        }
        if !fn_flag & (key != "".to_string()) {
            cmds.insert(key.clone(),val.clone());
            key = String::new();
            val = Vec::new();
        }
    }
    cmds
}
