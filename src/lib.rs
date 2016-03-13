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

pub fn index (
    _: &mut iron::request::Request,
) -> iron::IronResult<iron::response::Response> {
    let mut resp: iron::response::Response = iron::response::Response::new();

    use iron::Set;
    resp.set_mut(handlebars_iron::Template::new("index", ())).set_mut(iron::status::Ok);
    Ok(resp)
}

pub fn new () {
    let mut hbse = handlebars_iron::HandlebarsEngine::new2();

    hbse.add(std::boxed::Box::new (
        handlebars_iron::DirectorySource::new (
            "templates", ".hbs"
        )
    ));
    if let std::result::Result::Err(why) = hbse.reload() {
        use std::error::Error;
        panic!("{}", why.description());
    }

    let mut router = router::Router::new();

    router.get("/", index);

    let mut chain = iron::middleware::Chain::new(router);

    chain.link_after(hbse);

    println!("Server running at http://localhost:3000/");
    iron::Iron::new(chain).http("localhost:3000").unwrap();
}
