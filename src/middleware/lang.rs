// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;
extern crate l20n;
extern crate iron;

use std::io::Read;

/// The `ShareLang` structure defines and shares the lang by Arc.

pub struct ShareLang {
	pool: std::sync::Arc<Lang>,
}

impl ShareLang {

    /// The `new` constructor function returns the `ShareLang` from
    /// directory source's locale.

    pub fn new (
        locale: &str,
    ) -> std::io::Result<Self> {
        let lang = try!(Lang::new(locale));

        Ok(ShareLang::from_lang(lang))
    }

    /// The `from_lang` constructor function returns the `ShareLang` from
    /// Lang.

	pub fn from_lang (lang: Lang) -> Self {
		ShareLang {
			pool: std::sync::Arc::new(lang),
		}
	}

	/// The `get_table` function returns the good table from Arc
	/// according to the locale lang.

	pub fn get_table (
		&self,
		locale: String,
	) -> Option<std::collections::HashMap<String, String>> {
		self.pool.get_table(locale)
	}

	/// The `get_table_from_env` function returns the good table from Arc
	/// according to the locale lang and a environement.

	pub fn get_table_from_env (
		&self,
		locale: String,
		env: &std::collections::HashMap<String, String>,
	) -> Option<std::collections::HashMap<String, String>> {
		self.pool.get_table_from_env(locale, env)
	}
}

/// The `Key` sets our type to list of extension.

impl iron::typemap::Key for ShareLang {
	type Value = std::sync::Arc<Lang>;
}

/// The `BeforeMiddleware` links this middleware with the chain.

impl iron::BeforeMiddleware for ShareLang {
	fn before(&self, req: &mut iron::Request) -> iron::IronResult<()> {
		req.extensions.insert::<ShareLang>(self.pool.clone());
		Ok(())
	}
}

/// The `Lang` structure defines the international table of translation.

type Directionary = std::collections::HashMap<String, l20n::Locale>; // alias
type Environement = std::collections::HashMap<String, String>; // environement

pub struct Lang {
    table: Directionary, // locale, l20n's directionary.
	env: Environement, // environement.
}

impl Lang {

    /// The `new` constructor function returns the `Lang` from
    /// argument path.

    pub fn new (
        locale: &str,
    ) -> std::io::Result<Self> {
        let mut lang: Self = Lang {
            table: std::collections::HashMap::new(),
			env: std::collections::HashMap::new(),
        };

        try!(lang.add_directory(std::path::Path::new(locale)));
        Ok(lang)
    }

	/// The `get_table` function returns the good table according to
	/// the locale lang.

	pub fn get_table (
		&self,
		locale: String,
	) -> Option<std::collections::HashMap<String, String>> {
		self.get_table_from_env(locale, &self.env)
	}

	/// The `get_table_from_env` function returns the good table according to
	/// the locale lang and a environement.

	pub fn get_table_from_env (
		&self,
		locale: String,
		env: &std::collections::HashMap<String, String>,
	) -> Option<std::collections::HashMap<String, String>> {
		if let Some(l20n) = self.table.get(&locale) {
			match l20n.localize_data(&env) {
				Ok(data) => Some(data),
				Err(_) => None,
			}
		}
		else {
			None
		}
	}

    /// The `add_directory` function returns the `Lang` from
    /// unrecursively directory.

    fn add_directory (
        &mut self,
        path: &std::path::Path,
    ) -> std::io::Result<()> {
        if try!(std::fs::metadata(path)).is_dir() {
            for entry in try!(std::fs::read_dir(path)) {
                let dir: std::fs::DirEntry = try!(entry);

                if try!(dir.metadata()).is_file() {
                    let (key, value): (String, l20n::Locale) = try!(
                        self.parse_i20n_file(&dir.path())
                    );

                    self.table.insert(key, value);
                }
            }
        }
        Ok(())
    }

    /// The `parse_i20n_file` function returns the tuple
    /// key/value of locale/l20n from file.

    fn parse_i20n_file (
        &self,
        path: &std::path::Path,
    ) -> std::io::Result<(String, l20n::Locale)> {
        let mut name: String = String::new();
        let mut locale: l20n::Locale = l20n::Locale::new();

        if path.extension() == Some(std::ffi::OsStr::new("l20n")) {
            if let Some(file_name_ostr) = path.file_name() {
                if let Some(file_name) = file_name_ostr.to_str() {
					let (index, content): (String, String) = try!(
						self.parse_i20n(path, file_name)
					);

					locale.add_resource(&content).unwrap();
					name = index;
                }
            }
        }
        Ok((name, locale))
    }

	/// The `parse_i20n` function returns the tuple
    /// key/value of locale/string from file.

    #[allow(unused_must_use)]
    fn parse_i20n (
		&self,
		path: &std::path::Path,
		file_name: &str,
	) -> std::io::Result<(String, String)> {
		let mut buff: String = String::new();
		let mut file: std::fs::File = try!(std::fs::File::open (
			&path
		));

		file.read_to_string(&mut buff);
		Ok((
			file_name.chars().take_while(|&l|l != '.').collect::<String>(),
			buff
		))
	}
}
