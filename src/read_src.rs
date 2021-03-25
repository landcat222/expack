use std::path::PathBuf;
use std::collections::HashMap;
use crate::init;

type Src = Vec<Vec<String>>;

fn format(src: String) -> String {
    let pieces = src.split("\n");
    let mut result = String::new();
    for i in pieces {
        if i.trim() != "" {
            result = format!("{}{}\n",result,i.trim());
        }
    }
    result.trim().to_string()
}

fn conv_token(src: String) -> Src {
    let mut token: Src = Vec::new();
    for i in src.split("\n") {
        let mut tmp = Vec::new();
        for j in i.split(" ") {
            tmp.push(j.to_string());
        }
        token.push(tmp);
    }
    token
}

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
            if !fn_flag {
                tmp.push("$@".to_string());
            }
            val.push(tmp.clone());
        }
        if !fn_flag {
            cmds.insert(key.clone(),val.clone());
            key = String::new();
            val = Vec::new();
        }
    }
    cmds
}

pub(crate) fn get_src(src: init::EnvConf) -> HashMap<String,Src> {
    src_to_cmds(conv_token(format(src.get_src())))
}
