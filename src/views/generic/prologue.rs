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

pub fn prologue (
    req: &mut iron::request::Request,
) -> iron::IronResult<iron::response::Response> {
    let mut resp: iron::response::Response = iron::response::Response::new();
    let mut data: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    if let Some(shared_lang) = req.extensions.get::<middlewares::ShareLang>() {
        if let Some(lang) = shared_lang.get_table() {
            data.extend(lang);
        }
    }
    if let Some(shared_style) = req.extensions.get::<middlewares::ShareStyle>() {
        if let Some(sheet) = shared_style.get_sheet(&"book".to_string()) {
            data.insert("style".to_string(), sheet.clone());
        }
    }
    resp.set_mut(handlebars_iron::Template::new("prologue", data))
        .set_mut(iron::status::Ok);
    Ok(resp)
}
