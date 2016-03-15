// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;
extern crate l20n;

use std::io::Read;

type Directionary = std::collections::HashMap<String, l20n::Locale>;

/// The `Lang` structure defines the international table of translation.

pub struct Lang {
    table: Directionary, // locale, l20n's directionary.
}

impl Lang {

    /// The `from_directory` constructor function returns the `Lang` from
    /// argument path.

    pub fn new (
        locale: &str,
    ) -> std::io::Result<Self> {
        let mut lang: Self = Lang {
            table: std::collections::HashMap::new(),
        };

        try!(lang.from_directory(std::path::Path::new(locale)));
        Ok(lang)
    }

    /// The `from_directory` function returns the `Lang` from
    /// unrecursively directory.

    fn from_directory (
        &mut self,
        path: &std::path::Path,
    ) -> std::io::Result<()> {
        if try!(std::fs::metadata(path)).is_dir() {
            for entry in try!(std::fs::read_dir(path)) {
                let dir: std::fs::DirEntry = try!(entry);

                if try!(dir.metadata()).is_file() {
                    let (key, value): (String, l20n::Locale) = try!(
                        self.parse_i20n(&dir.path())
                    );

                    self.table.insert(key, value);
                }
            }
        }
        Ok(())
    }

    /// The `parse_i20n` function returns the tuple
    /// key/value of locale/l20n from file.

    #[allow(unused_must_use)]
    fn parse_i20n (
        &self,
        path: &std::path::Path,
    ) -> std::io::Result<(String, l20n::Locale)> {
        let mut name: String = String::new();
        let mut locale: l20n::Locale = l20n::Locale::new();

        if path.extension() == Some(std::ffi::OsStr::new("l20n")) {
            if let Some(file_name_ostr) = path.file_name() {
                if let Some(file_name) = file_name_ostr.to_str() {
                    let mut buff: String = String::new();
                    let mut file: std::fs::File = try!(std::fs::File::open (
                        &path
                    ));

                    file.read_to_string(&mut buff);
                    locale.add_resource(&buff).unwrap();
                    name.push_str(&file_name.chars().take_while(|&l|
                        l != '.'
                    ).collect::<String>());
                }
            }
        }
        Ok((name, locale))
    }
}
