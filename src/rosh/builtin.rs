use super::prelude::*;
type Builtin = fn(Vec<String>);
type BuiltinMap = HashMap<String, Builtin>;

pub mod functions {
    pub fn exit(args: Vec<String>) {
        // need to read exit code, however need a proper lexer and parser
        std::process::exit(0);
    }

    pub fn cd(args: Vec<String>) {
        let dir = std::path::Path::new(&args[1]);
        std::env::set_current_dir(&dir).unwrap();
    }
}

pub fn get_map() -> Option<BuiltinMap> {
    let mut builtin_map: BuiltinMap = HashMap::new();
    builtin_map.insert("exit".to_string(), functions::exit);
    builtin_map.insert("cd".to_string(), functions::cd);
    Some(builtin_map)
}
