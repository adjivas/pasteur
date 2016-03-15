// @adjivas - github.com/adjivas. See the LICENSE
// file at the top-level directory of this distribution and at
// https://github.com/adjivas/pasteur
//
// This file may not be copied, modified, or distributed
// except according to those terms.

extern crate std;

/// The `Protocol` enumerator defines the list of
/// server's protocol available.

pub enum Protocol {
    HTTP,
    HTTPS,
}

impl Protocol {

    /// The `from_str` function constructor returns the
    /// `Protocol` enumerator.

    pub fn from_str (
        protocol: &str,
    ) -> Option<Self> {
        match protocol {
            "http" => Some(Protocol::HTTP),
            "https" => Some(Protocol::HTTPS),
            _ => None,
        }
    }
}

/// The `Display` print returns the `Protocol` String associed.

impl std::fmt::Display for Protocol {
    fn fmt (
        &self,
        f: &mut std::fmt::Formatter
    ) -> Result<(), std::fmt::Error> {
        match self {
            &Protocol::HTTP => {
                let _ = write!(f, "http");
            }
            &Protocol::HTTPS => {
                let _ = write!(f, "https");
            }
        };
        Ok(())
    }
}

/// The `Default` construtor returns the https' protocol.

impl Default for Protocol {
    fn default () -> Self {
        Protocol::HTTPS
    }
}
