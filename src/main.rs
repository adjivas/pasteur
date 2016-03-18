// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate clap;
extern crate pasteur;

/// Default* const arguments defined by CLI.

const DEFAULT_TEMPLATE: &'static str = "etc/templates";
const DEFAULT_LOCALE: &'static str = "etc/locales";
const DEFAULT_STYLE: &'static str = "etc/stylesheets";
const DEFAULT_CERT: &'static str = "etc/ca/cert.pem";
const DEFAULT_KEY: &'static str = "etc/ca/key.pem";
const DEFAULT_PROTOCOL: &'static str = "https";
const DEFAULT_ADDRESS: &'static str = "localhost";
const DEFAULT_SOCKET: &'static str = "3000";

/// The `main` function parses the arguments and
/// instantiates the (http | https) server.

pub fn main () {
    let yaml = load_yaml!("cli.yml");
    let options = clap::App::from_yaml(yaml).get_matches();

    pasteur::new (
        options.value_of("template").unwrap_or(DEFAULT_TEMPLATE),
        options.value_of("locale").unwrap_or(DEFAULT_LOCALE),
        options.value_of("style").unwrap_or(DEFAULT_STYLE),
        options.value_of("cert").unwrap_or(DEFAULT_CERT),
        options.value_of("key").unwrap_or(DEFAULT_KEY),
        pasteur::protocol::Protocol::from_str (
            options.value_of("protocol").unwrap_or(DEFAULT_PROTOCOL)
        ).unwrap(),
        &format!("{}:{}",
            options.value_of("address").unwrap_or(DEFAULT_ADDRESS),
            options.value_of("socket").unwrap_or(DEFAULT_SOCKET)
        ),
    );
}
