// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use paracell_util_macro::AsVariant;

#[derive(Clone, Debug)]
pub struct Nat {
    pub val: u128,
}

#[derive(Clone, Debug)]
pub struct Ident {
    pub lit: String,
}

#[derive(Clone, Debug)]
pub enum UnaryOperator {
    Invert,
    Not,
}

#[derive(Clone, Debug)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub ident: Ident,
    pub ty: Item,
}

#[derive(Clone, Debug)]
pub struct Variant {
    pub ident: Ident,
    pub ty: Item,
}

#[derive(Clone, Debug)]
pub enum ListLead {
    Union,
    Record,
    Func,
    Match,

    Apply,
    Return,
}

#[derive(Clone, Debug)]
pub struct List {
    pub lead: ListLead,
    pub items: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct UnaryOpExpr {
    pub op: UnaryOperator,
    pub expr: Item,
}

#[derive(Clone, Debug)]
pub struct BinaryOpExpr {
    pub op: BinaryOperator,
    pub left: Item,
    pub right: Item,
}

#[derive(Clone, Debug)]
pub struct LetDecl {
    pub ident: Ident,
    pub expr: Item,
}

#[derive(Clone, Debug)]
pub struct VarDecl {
    pub ident: Ident,
    pub expr: Item,
}

#[derive(Clone, Debug, AsVariant)]
pub enum Item {
    Nat(Nat),
    Ident(Ident),

    Field(Box<Field>),
    Variant(Box<Variant>),

    LetDecl(Box<LetDecl>),
    VarDecl(Box<VarDecl>),

    UnaryOpExpr(Box<UnaryOpExpr>),
    BinaryOpExpr(Box<BinaryOpExpr>),

    List(List),
}
