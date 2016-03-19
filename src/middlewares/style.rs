// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use std::io::Read;

use sass_rs;
use iron;

/// The `ShareStyle` structure defines and shares the style by Arc.

pub struct ShareStyle {
	source: std::sync::Arc<Style>,
}

impl ShareStyle {

    /// The `new` constructor function returns the `ShareStyle` from
    /// directory source's locale.

    pub fn new (
        locale_source: &str,
    ) -> std::io::Result<Self> {
        let style = try!(Style::new(locale_source));

        Ok(ShareStyle::from_style(style))
    }

    /// The `from_style` constructor function returns the `ShareStyle` from
    /// Style.

	pub fn from_style (style: Style) -> Self {
		ShareStyle {
			source: std::sync::Arc::new(style),
		}
	}
}

/// The `Key` sets our type to list of extension.

impl iron::typemap::Key for ShareStyle {
	type Value = DesctiptionStyle;
}

/// The `BeforeMiddleware` links this middleware with the chain.

impl iron::BeforeMiddleware for ShareStyle {

	/// The `before` constructor function sets the key from typemap with
	/// the interface `ShareStyle`.

	fn before (&self, req: &mut iron::Request) -> iron::IronResult<()> {
		req.extensions.insert::<ShareStyle> (
			DesctiptionStyle::new (
				self.source.clone(),
			)
		);
		Ok(())
	}
}

/// The `DesctiptionStyle` structure defines an interface
/// to get sentence internationalized.

pub struct DesctiptionStyle {
	style: std::sync::Arc<Style>,
}

impl DesctiptionStyle {

	/// The `new` constructor function returns the `DesctiptionStyle` to store
	/// the style header.

	pub fn new (
		style: std::sync::Arc<Style>,
	) -> Self {
		DesctiptionStyle {
			style: style,
		}
	}

	pub fn get_sheet (
		&self,
		sheet: &String,
	) -> Option<&String> {
		self.style.get_sheet(sheet)
	}
}


/// The `Style` structure defines the international table of translation.

pub struct Style {
    sheet: std::collections::HashMap<String, String>, // name, sheet.
}

impl Style {

    /// The `new` constructor function returns the `Style` from
    /// argument path.

    pub fn new (
        style_source: &str,
    ) -> std::io::Result<Self> {
        let mut style: Self = Style {
            sheet: std::collections::HashMap::new(),
        };

        try!(style.add_from_directory(std::path::Path::new(style_source)));
        Ok(style)
    }


	/// The `get_sheet` function returns the good css's sheet according to
	/// the name.

	pub fn get_sheet (
		&self,
		style: &String,
	) -> Option<&String> {
		self.sheet.get(style)
	}

    /// The `add_from_directory` function returns the `Style` from
    /// unrecursively directory.

    fn add_from_directory (
        &mut self,
        path: &std::path::Path,
    ) -> std::io::Result<()> {
        if try!(std::fs::metadata(path)).is_dir() {
            for entry in try!(std::fs::read_dir(path)) {
                let dir: std::fs::DirEntry = try!(entry);

                if try!(dir.metadata()).is_file() {
                    let (key, value): (String, String) = try!(
                        self.parse_style_file(&dir.path())
                    );

                    self.sheet.insert(key, value);
                }
            }
        }
        Ok(())
    }

    /// The `parse_style_file` function returns the tuple
    /// key/value of locale/l20n from file.

    fn parse_style_file (
        &self,
        path: &std::path::Path,
    ) -> std::io::Result<(String, String)> {
        let mut input: String = String::new();
        let mut sheet: String = String::new();

        if path.extension() == Some(std::ffi::OsStr::new("scss"))
		|| path.extension() == Some(std::ffi::OsStr::new("sass")) {
            if let Some(file_name_ostr) = path.file_name() {
                if let Some(file_name) = file_name_ostr.to_str() {
					input = file_name.chars().take_while(|&l|
						l != '.'
					).collect::<String>();
					sheet = match sass_rs::sass_context::SassFileContext::new (
						&format!("{}", path.display())
					).compile() {
						Ok(sheet) => sheet,
						Err(why) => return Err(std::io::Error::new(std::io::ErrorKind::Other, why)),
					};
                }
            }
        }
		Ok((input, sheet))
    }
}
