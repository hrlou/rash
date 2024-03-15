mod rosh {
    use std::collections::HashMap;
    
    pub fn rosh() {
        let path_map = read_path().unwrap();
        let builtin_map = builtin().unwrap();
        loop {
            if let Some(args) = self::input() {
                if path_map.contains_key(&args[0]) {
                    self::execute(path_map[&args[0]].clone(), args);
                } else if builtin_map.contains_key(&args[0]) {
                    let exec = builtin_map[&args[0]];
                    exec(args);
                } else {
                    eprintln!("rosh: command not found: {}", &args[0])
                }
            }
        }
        
    }

    pub fn builtin() -> Option<HashMap<String, fn(Vec<String>)>> {
        type Exec = fn(Vec<String>);
        type ExecMap = HashMap<String, Exec>;
        let mut builtin_map: ExecMap = HashMap::new();
        builtin_map.insert("exit".to_string(), |_: Vec<String>| { std::process::exit(0) });
        builtin_map.insert("cd".to_string(), |args: Vec<String>| { 
            let dir = std::path::Path::new(&args[1]);
            std::env::set_current_dir(&dir).unwrap();
        });
        Some(builtin_map)
    }

    pub fn prompt() {
        let prompt = std::env::var("PS1").unwrap_or("rosh> ".to_string());
        eprint!("{}", prompt);
    }

    pub fn input() -> Option<Vec<String>> {
        self::prompt();
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);
        buf.pop();
        let split: Vec<&str> = buf.split(' ').collect();
        if buf.len() < 1 {
            None
        } else {
            let mut out: Vec<String> = Vec::new();
            for i in split {
                out.push(i.to_string());
            }
            Some(out)
        }
    }

    pub fn execute(path: String, args: Vec<String>) {
        let mut args = args.clone();
        args.remove(0);
        let mut child = std::process::Command::new(path)
            .args(args)
            .spawn()
            .unwrap();
        let _ = child.wait();
    }

    pub fn read_path() -> Option<HashMap<String, String>> {
        let mut map: HashMap<String, String> = HashMap::new();
        let global_path = std::env::var("PATH").unwrap();
        let paths: Vec<&str> = global_path.split(':').collect();
        for search_dir in paths {
            let paths = std::fs::read_dir(search_dir).unwrap();
            for bin in paths {
                let path = bin.unwrap().path();
                let path_string = String::from(path.to_str().unwrap());
                let path_name_string = String::from(path.file_name().unwrap().to_str().unwrap());
                map.insert(path_name_string, path_string);
            }
        }
        Some(map)
    }
}

fn main() {
    rosh::rosh()
}
