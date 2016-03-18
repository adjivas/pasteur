// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate sass_rs;

pub fn main() {
    let input = "etc/stylesheets/sample.scss";
    let mut file_context = sass_rs::sass_context::SassFileContext::new(&input);

    file_context.compile().unwrap();
}
