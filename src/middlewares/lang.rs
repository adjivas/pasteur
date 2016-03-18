// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use std::io::Read;

use l20n;
use iron;

/// The `ShareLang` structure defines and shares the lang by Arc.

pub struct ShareLang {
	source: std::sync::Arc<Lang>,
}

impl ShareLang {

    /// The `new` constructor function returns the `ShareLang` from
    /// directory source's locale.

    pub fn new (
        locale_source: &str,
    ) -> std::io::Result<Self> {
        let lang = try!(Lang::new(locale_source));

        Ok(ShareLang::from_lang(lang))
    }

    /// The `from_lang` constructor function returns the `ShareLang` from
    /// Lang.

	pub fn from_lang (lang: Lang) -> Self {
		ShareLang {
			source: std::sync::Arc::new(lang),
		}
	}
}

/// The `Key` sets our type to list of extension.

impl iron::typemap::Key for ShareLang {
	type Value = DesctiptionLang;
}

/// The `BeforeMiddleware` links this middleware with the chain.

impl iron::BeforeMiddleware for ShareLang {

	/// The `before` constructor function sets the key from typemap with
	/// the interface `ShareLang`.

	fn before (&self, req: &mut iron::Request) -> iron::IronResult<()> {
		if let Some(accept) = req.headers.get::<iron::headers::AcceptLanguage>() {
			req.extensions.insert::<ShareLang> (
				DesctiptionLang::new (
					self.source.clone(),
					(accept).iter().filter_map(|lang|
						match (&lang.item.language, &lang.item.region) {
							(&Some(ref language), &Some(ref region)) => {
								let lang: &String = &format!("{}-{}", &language, &region);
								if self.source.check_get(lang) {
									Some(lang.clone())
								}
								else {
									None
								}
							},
							_ => None,
						}
					).collect::<Vec<String>>()
				)
			);
		}
		Ok(())
	}
}

/// The `DesctiptionLang` structure defines an interface
/// to get sentence internationalized.

pub struct DesctiptionLang {
	lang: std::sync::Arc<Lang>,
	locales: Vec<String>, // languages.
	locale: String, // default lang.
}

impl DesctiptionLang {

	/// The `new` constructor function returns the `DesctiptionLang` to store
	/// the lang header.

	pub fn new (
		lang: std::sync::Arc<Lang>,
		locales: Vec<String>,
	) -> Self {
		DesctiptionLang {
			lang: lang,
			locales: locales.clone(),
			locale: if let Some(locale) = locales.get(0) {
				locale.clone()
			} else {
				"en-US".to_string()
			},
		}
	}

	pub fn get_table (
		&self,
	) -> Option<std::collections::HashMap<String, String>> {
		self.lang.get_table(&self.locale)
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
        locale_source: &str,
    ) -> std::io::Result<Self> {
        let mut lang: Self = Lang {
            table: std::collections::HashMap::new(),
			env: std::collections::HashMap::new(),
        };

        try!(lang.add_directory(std::path::Path::new(locale_source)));
        Ok(lang)
    }

	/// The `get_table` function returns the good table according to
	/// the locale lang.

	pub fn get_table (
		&self,
		locale: &String,
	) -> Option<std::collections::HashMap<String, String>> {
		self.get_table_from_env(locale, &self.env)
	}


	/// The `get_table_from_env` function returns the good table according to
	/// the locale lang and a environement.

	pub fn get_table_from_env (
		&self,
		locale: &String,
		env: &std::collections::HashMap<String, String>,
	) -> Option<std::collections::HashMap<String, String>> {
		if let Some(l20n) = self.table.get(locale) {
			match l20n.localize_data(&env) {
				Ok(data) => Some(data),
				Err(_) => None,
			}
		}
		else {
			None
		}
	}

	/// The `check_get` function returns a boolean
	/// for `locale` language member from `table`.

	pub fn check_get (
		&self,
		locale: &String,
	) -> bool {
		self.table.get(locale).is_some()
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
					let (name_parsed, content): (String, String) = try!(
						self.parse_i20n(path, file_name)
					);

					locale.add_resource(&content);
					name = name_parsed;
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
		let name = file_name.chars().take_while(|&l| l != '.').collect::<String>();
		let mut buff: String = String::new();
		let mut file: std::fs::File = try!(std::fs::File::open(&path));

		try!(file.read_to_string(&mut buff));
		Ok((name, buff))
	}
}
