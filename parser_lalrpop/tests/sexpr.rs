// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use paracell_parser_lalrpop::sexpr::ast::*;
use paracell_parser_lalrpop::sexpr::grammar;

#[test]
fn test_parse_item() {
    let mut v = grammar::ItemParser::new().parse("
        (record
            (field A Nat)
            (field B Nat))
    ").unwrap().as_List().unwrap();

    match v.lead {
        ListLead::Record => {}
        _ => panic!(),
    }

    let field2 = v.items.pop().unwrap().as_Field().unwrap();
    let field1 = v.items.pop().unwrap().as_Field().unwrap();

    assert_eq!(field1.ident.lit, "A");
    assert_eq!(field2.ident.lit, "B");

    assert_eq!(field1.ty.as_Ident().unwrap().lit, "Nat");
    assert_eq!(field2.ty.as_Ident().unwrap().lit, "Nat");
}