// Copyright 2017 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate serde_derive;

#[derive(Serialize)] //~ ERROR: 12:10: 12:19: variant `Struct` cannot have both #[serde(serialize_with)] and a field `f1` marked with #[serde(skip_serializing)]
enum Enum {
    #[serde(serialize_with = "serialize_some_other_variant")]
    Struct {
        #[serde(skip_serializing)]
        f1: String,
        f2: u8,
    },
}

fn main() { }
