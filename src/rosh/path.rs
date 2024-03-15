use super::prelude::*;

pub fn get_map() -> Option<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();
    let global_path = std::env::var("PATH").unwrap();
    let paths: Vec<&str> = global_path.split(':').collect();
    for search_dir in paths {
        if let Ok(paths) = std::fs::read_dir(search_dir) {
            for bin in paths {
                let path = bin.unwrap().path();
                let path_string = String::from(path.to_str().unwrap());
                let path_name_string =
                    String::from(path.file_name().unwrap().to_str().unwrap());
                map.insert(path_name_string, path_string);
            }
        }
    }
    Some(map)
}
