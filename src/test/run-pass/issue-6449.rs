// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

enum Foo {
    Bar(int),
    Baz,
}

enum Other {
    Other1(Foo),
    Other2(Foo, Foo),
}

fn main() {
    match Baz {
        ::Bar(3) => panic!(),
        ::Bar(_) if false => panic!(),
        ::Bar(..) if false => panic!(),
        ::Bar(_n) => panic!(),
        ::Baz => {}
    }
    match Bar(3) {
        ::Bar(3) => {}
        ::Bar(_) if false => panic!(),
        ::Bar(..) if false => panic!(),
        ::Bar(_n) => panic!(),
        ::Baz => panic!(),
    }
    match Bar(4) {
        ::Bar(3) => panic!(),
        ::Bar(_) if false => panic!(),
        ::Bar(..) if false => panic!(),
        ::Bar(n) => assert_eq!(n, 4),
        ::Baz => panic!(),
    }

    match Other1(Baz) {
        ::Other1(::Baz) => {}
        ::Other1(::Bar(_)) => {}
        ::Other2(::Baz, ::Bar(_)) => {}
        ::Other2(::Bar(..), ::Baz) => {}
        ::Other2(..) => {}
    }
}
