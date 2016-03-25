// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

use middlewares;

use handlebars_iron;
use iron;
use iron::modifier::Set;

pub fn get (
    req: &iron::request::Request,
) -> Option<handlebars_iron::Template> {
    let mut data: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut result: Option<handlebars_iron::Template> = None;

    if let (Some(shared_lang), Some(shared_style)) = (
        req.extensions.get::<middlewares::ShareLang>(),
        req.extensions.get::<middlewares::ShareStyle>()
    ) {
        if let (Some(lang), Some(sheet)) = (
            shared_lang.get_table(),
            shared_style.get_sheet(&"book".to_string())
        ) {
            data.extend(lang);
            data.insert("style".to_string(), sheet.clone());
            result = Some(handlebars_iron::Template::new("prologue", data));
        }
    }
    result
}

pub fn index (
    req: &mut iron::request::Request,
) -> iron::IronResult<iron::response::Response> {
    let mut resp: iron::response::Response = iron::response::Response::new();

    if let Some(template) = self::get(req) {
        resp.set_mut(template)
            .set_mut(iron::status::Ok);
    }
    Ok(resp)
}
