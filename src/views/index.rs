// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

use middleware;

use handlebars_iron;
use iron;
use iron::modifier::Set;

pub fn index (
    req: &mut iron::request::Request,
) -> iron::IronResult<iron::response::Response> {
    let mut resp: iron::response::Response = iron::response::Response::new();

    if let Some(shared_lang) = req.extensions.get::<middleware::ShareLang>() {
        if let Some(lang) = shared_lang.get_table() {
            resp.set_mut(handlebars_iron::Template::new("index", lang))
                .set_mut(iron::status::Ok);
        }
    }
    Ok(resp)
}
