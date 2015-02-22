// Copyright 2012-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(plugin, rustc_private)]
#![plugin(quote_macros)]

extern crate syntax;

use syntax::ext::base::ExtCtxt;
use syntax::ext::expand;
use syntax::parse;
use syntax::print::pprust;

fn make_ext_ctxt(sess: &parse::ParseSess) -> ExtCtxt {
    let info = syntax::codemap::ExpnInfo {
        call_site: syntax::codemap::DUMMY_SP,
        callee: syntax::codemap::NameAndSpan {
            name: "test".to_string(),
            format: syntax::codemap::MacroAttribute,
            span: None
        }
    };

    let cfg = vec![];
    let ecfg = expand::ExpansionConfig {
        crate_name: String::new(),
        features: None,
        recursion_limit: 64,
    };

    let mut cx = ExtCtxt::new(&sess, cfg, ecfg);
    cx.bt_push(info);

    cx
}

#[test]
fn test_quote_expr() {
    let sess = parse::new_parse_sess();
    let cx = make_ext_ctxt(&sess);

    let expr = quote_expr!(&cx, 23);
    assert_eq!(pprust::expr_to_string(&expr), "23");

    let value = 24;
    let expr = quote_expr!(&cx, $value);
    assert_eq!(pprust::expr_to_string(&expr), "24i32");
}

#[test]
fn test_quote_ty() {
    let sess = parse::new_parse_sess();
    let cx = make_ext_ctxt(&sess);

    let ty = quote_ty!(&cx, isize);
    assert_eq!(pprust::ty_to_string(&ty), "isize");
}

#[test]
fn test_quote_item() {
    let sess = parse::new_parse_sess();
    let cx = make_ext_ctxt(&sess);

    let item = quote_item!(&cx, static x : int = 10;).unwrap();
    assert_eq!(pprust::item_to_string(&item), "static x: int = 10;");
}

#[test]
fn test_quote_stmt() {
    let sess = parse::new_parse_sess();
    let cx = make_ext_ctxt(&sess);

    let stmt = quote_stmt!(&cx, let x = 20;);
    assert_eq!(pprust::stmt_to_string(&stmt), "let x = 20;");
}

#[test]
fn test_quote_pat() {
    let sess = parse::new_parse_sess();
    let cx = make_ext_ctxt(&sess);

    let pat = quote_pat!(&cx, Some(_));
    assert_eq!(pprust::pat_to_string(&pat), "Some(_)");
}

#[test]
fn test_quote_arm() {
    let sess = parse::new_parse_sess();
    let cx = make_ext_ctxt(&sess);

    let arm = quote_arm!(&cx, (ref x, ref y) => (x, y),);
    assert_eq!(pprust::arm_to_string(&arm), " (ref x, ref y) => (x, y),");
}
