// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

//! # Pasteur - Iron
//!
//! [![travis-badge][]][travis] [![coverage-badge][]][coverage] [![clippy-badge][]][clippy] [![docs-badge][]][docs] [![license-badge][]][license] [![gitter-badge][]][gitter]
//!
//! [license-badge]: https://img.shields.io/crates/l/cublas.svg?style=flat-square
//! [license]: https://github.com/adjivas/pasteur/blob/master/README.md#license
//! [docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
//! [docs]: http://adjivas.github.io/pasteur/pasteur
//! [gitter-badge]: https://badges.gitter.im/adjivas/pasteur.svg?style=flat-square
//! [gitter]: https://gitter.im/adjivas/pasteur?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge
//! [travis-badge]: https://travis-ci.org/adjivas/pasteur.svg?branch=master&style=flat-square
//! [travis]: https://travis-ci.org/adjivas/pasteur
//! [coverage-badge]: https://coveralls.io/repos/github/adjivas/pasteur/badge.svg?branch=master&style=flat-square
//! [coverage]: https://coveralls.io/github/adjivas/pasteur?branch=master
//! [clippy-badge]: https://clippy.bashy.io/github/adjivas/pasteur/master/badge.svg?style=flat-square
//! [clippy]: https://clippy.bashy.io/github/adjivas/pasteur/master/log

extern crate sass_rs;
extern crate handlebars_iron;
extern crate router;
extern crate iron;
extern crate l20n;

pub mod protocol;
mod views;
mod middlewares;

use std::error::Error;

/// The `new` function instancies the server.

pub fn new (
    template: &str,
    locale: &str,
    style: &str,
    cert: &str,
    key: &str,
    protocol: protocol::Protocol,
    address: &str,
) {
    let mut hbse = handlebars_iron::HandlebarsEngine::new2();

    hbse.add(std::boxed::Box::new (
        handlebars_iron::DirectorySource::new (
            template, ".hbs"
        )
    ));
    if let std::result::Result::Err(why) = hbse.reload() {
        panic!("{}", why.description());
    }

    let mut router = router::Router::new();

    router.get("/", views::index);
    router.get("/book", views::generic::prologue);

    let mut chain = iron::middleware::Chain::new(router);

    chain.link_before(middlewares::ShareLang::new(locale).unwrap());
    chain.link_before(middlewares::ShareStyle::new(style).unwrap());
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
