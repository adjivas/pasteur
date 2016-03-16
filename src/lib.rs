// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate rustc_serialize;
extern crate handlebars_iron;
extern crate handlebars;
extern crate router;
extern crate iron;

pub mod protocol;
mod middleware;

use std::error::Error;
use iron::modifier::Set;
use iron::headers::AcceptLanguage;

pub fn index (
    req: &mut iron::request::Request,
) -> iron::IronResult<iron::response::Response> {
    println!("{:?}", req.headers.get_raw("accept-language") );

    match req.extensions.get::<middleware::ShareLang>() {
        Some(lang) => println!("work {:?}", lang.get_table("en-US".to_string())),
        None => println!("fail"),
    }

    let mut resp: iron::response::Response = iron::response::Response::new();

    resp.set_mut(handlebars_iron::Template::new("index", ())).set_mut(iron::status::Ok);
    Ok(resp)
}

/// The `new` function instantiates the server.

pub fn new (
    template: &str,
    locale: &str,
    cert: &str,
    key: &str,
    protocol: protocol::Protocol,
    address: &str,
) {
    let mut hbse = handlebars_iron::HandlebarsEngine::new2();
    let lang = middleware::ShareLang::new(locale).unwrap();

    hbse.add(std::boxed::Box::new (
        handlebars_iron::DirectorySource::new (
            template, ".hbs"
        )
    ));
    if let std::result::Result::Err(why) = hbse.reload() {
        panic!("{}", why.description());
    }

    let mut router = router::Router::new();

    router.get("/", index);

    let mut chain = iron::middleware::Chain::new(router);

    chain.link_before(lang);
    chain.link_after(hbse);

    println!("Server running at {}://{}/", protocol, address);
    match protocol {
        protocol::Protocol::HTTP => iron::Iron::new(chain).http(address),
        protocol::Protocol::HTTPS => iron::Iron::new(chain).https (
          address,
          std::path::PathBuf::from(cert),
          std::path::PathBuf::from(key)
        ),
    }.unwrap();
}
