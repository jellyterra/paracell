// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use paracell_parser_lalrpop::flow::ast::*;
use paracell_parser_lalrpop::flow::grammar;

#[test]
fn test_parse_nat() {
    let hex = grammar::NatParser::new().parse("0xEF").unwrap().val;
    let dec = grammar::NatParser::new().parse("1024").unwrap().val;
    let oct = grammar::NatParser::new().parse("0o644").unwrap().val;
    let bin = grammar::NatParser::new().parse("0b1001").unwrap().val;
    assert_eq!(hex, 0xEF);
    assert_eq!(dec, 1024);
    assert_eq!(oct, 0o644);
    assert_eq!(bin, 0b1001);
}

#[test]
fn test_parse_record() {
    let mut s = grammar::ItemParser::new().parse("
        record {
            A: Nat,
            B: Nat
        }
    ").unwrap().expect_semantic_type().unwrap().as_Record().unwrap();

    let f2 = s.fields.pop().unwrap();
    let f1 = s.fields.pop().unwrap();

    assert_eq!(f1.ident, "A");
    assert_eq!(f2.ident, "B");
}

#[test]
fn test_parse_union() {
    let mut s = grammar::ItemParser::new().parse("
        union {
            A: Nat,
            B: Nat
        }
    ").unwrap().expect_semantic_type().unwrap().as_Union().unwrap();

    let v2 = s.variants.pop().unwrap();
    let v1 = s.variants.pop().unwrap();

    assert_eq!(v1.ident, "A");
    assert_eq!(v2.ident, "B");
}

#[test]
fn test_parse_unary() {
    let mut s = grammar::ItemParser::new().parse("
        ~Bit
    ").unwrap().expect_semantic_expr().unwrap().as_Apply().unwrap();

    let f = s.params.fields.pop().unwrap();

    assert_eq!(s.func.as_Ident().unwrap(), UnaryOperator::Invert.to_literal());
    assert_eq!(f.expr.as_Ident().unwrap(), "Bit");
}

#[test]
fn test_parse_binary() {
    let mut s = grammar::ItemParser::new().parse("
        1 + 2
    ").unwrap().expect_semantic_expr().unwrap().as_Apply().unwrap();

    let f2 = s.params.fields.pop().unwrap();
    let f1 = s.params.fields.pop().unwrap();

    assert_eq!(s.func.as_Ident().unwrap(), BinaryOperator::Add.to_literal());
    assert_eq!(f1.expr.as_Nat().unwrap().val, 1);
    assert_eq!(f2.expr.as_Nat().unwrap().val, 2);
}

#[test]
fn test_parse_let() {
    let s = grammar::ItemParser::new().parse("
        let v = 1 + 2
    ").unwrap().expect_semantic_decl().unwrap().as_Let().unwrap();

    assert_eq!(s.ident, "v");

    let mut arith = s.expr.as_Apply().unwrap();

    let f2 = arith.params.fields.pop().unwrap();
    let f1 = arith.params.fields.pop().unwrap();

    assert_eq!(arith.func.as_Ident().unwrap(), BinaryOperator::Add.to_literal());
    assert_eq!(f1.expr.as_Nat().unwrap().val, 1);
    assert_eq!(f2.expr.as_Nat().unwrap().val, 2);
}

#[test]
fn test_parse_func() {
    let mut s = grammar::ItemParser::new().parse("
        fun (a: Nat, b: Nat) -> Nat {
            let v = a + b;
            v
        }
    ").unwrap().expect_semantic_expr().unwrap().as_Func().unwrap();

    let result = s.block.stmts.pop().unwrap().as_Expr().unwrap().as_Ident().unwrap();

    assert_eq!(result, "v");

    let let_decl = s.block.stmts.pop().unwrap().as_Decl().unwrap().as_Let().unwrap();
}

#[test]
fn test_parse_apply() {
    let mut s = grammar::ItemParser::new().parse("
        Invoke(1, 2, 3 + 4)
    ").unwrap().expect_semantic_expr().unwrap().as_Apply().unwrap();

    assert_eq!(s.func.as_Ident().unwrap(), "Invoke");
}

#[test]
fn test_parse_match() {
    let mut s = grammar::ItemParser::new().parse("
        match nat {
            1 => 2,
            3 => 4
        }
    ").unwrap().expect_semantic_expr().unwrap().as_Match().unwrap();

    let c2 = s.cases.pop().unwrap();
    let c1 = s.cases.pop().unwrap();

    assert_eq!(c1.pattern.as_Nat().unwrap().val, 1);
    assert_eq!(c2.pattern.as_Nat().unwrap().val, 3);
    assert_eq!(c1.expr.as_Nat().unwrap().val, 2);
    assert_eq!(c2.expr.as_Nat().unwrap().val, 4);
}
