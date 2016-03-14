// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate walkdir;
extern crate l20n;

pub fn new (
    path: &str,
) {
    let mut locale = l20n::Locale::new();

    for entry in walkdir::WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let path: &std::path::Path = entry.path();
        if path.extension().unwrap_or("".as_ref()) == "l20n" {
            match std::fs::File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}",
                    path.display(),
                    std::error::Error::description(&why)
                ),
                Ok(mut file) => {
                    use std::io::Read;
                    let mut buff = String::new();

                    file.read_to_string(&mut buff);
                    locale.add_resource(&buff).unwrap();
                },
            };
        }
    }


    let mut env = std::collections::HashMap::new();
    env.insert("name", "Rust");
    let strs: std::collections::HashMap<String, String> = locale.localize_data(env).unwrap();
    println!("{}", strs["hi"]);
}
