// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use paracell_util_macro::{AsVariant, ToLiteral};

#[derive(Clone, Debug)]
pub struct Nat {
    pub val: u128,
}

#[derive(Clone, Debug)]
pub struct Ident {
    pub lit: String,
}

#[derive(Clone, Debug)]
pub struct IdentItem {
    pub ident: Ident,
    pub item: Item,
}

#[derive(Clone, Debug)]
pub struct RecordType {
    pub fields: Vec<IdentItem>,
}

#[derive(Clone, Debug)]
pub struct UnionType {
    pub variants: Vec<IdentItem>,
}

#[derive(Clone, Debug)]
pub struct FuncType {
    pub param_tuple: Tuple,
    pub result_ty: Item,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub elems: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct Func {
    pub ty: FuncType,
    pub block: Block,
}

#[derive(Clone, Debug)]
pub struct RecordExpr {
    pub fields: Vec<IdentItem>,
}

#[derive(Clone, Debug)]
pub struct ApplyExpr {
    pub func: Item,
    pub params: Tuple,
}

#[derive(Clone, Debug)]
pub struct Case {
    pub pattern: Item,
    pub expr: Item,
}

#[derive(Clone, Debug)]
pub struct Match {
    pub expr: Item,
    pub cases: Vec<Case>,
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

#[derive(Clone, Debug)]
pub struct TypeAliasDecl {
    pub ident: Ident,
    pub ty: Item,
}

#[derive(Clone, Debug)]
pub struct Tuple {
    pub elems: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct TypeTuple {
    pub elems: Vec<Item>,
}

#[derive(Clone, Debug)]
pub struct Select {
    pub expr: Item,
    pub ident: Ident,
}

#[derive(Clone, Debug)]
pub struct Pipe {
    pub from: Item,
    pub to: Item,
}

#[derive(Clone, Debug, ToLiteral)]
pub enum UnaryOperator {
    #[literal = "~"]
    Invert,
    #[literal = "!"]
    Not,
}

#[derive(Clone, Debug, ToLiteral)]
pub enum BinaryOperator {
    #[literal = "+"]
    Add,
    #[literal = "-"]
    Sub,
    #[literal = "*"]
    Mul,
    #[literal = "/"]
    Div,
    #[literal = "%"]
    Mod,
    #[literal = "&"]
    And,
    #[literal = "|"]
    Or,
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

#[derive(Clone, Debug, AsVariant)]
pub enum Item {
    Nat(Nat),
    Ident(Ident),
    Tuple(Tuple),
    Block(Box<Block>),
    Func(Box<Func>),
    Match(Box<Match>),
    TypeTuple(Box<TypeTuple>),

    RecordType(Box<RecordType>),
    UnionType(Box<UnionType>),
    FuncType(Box<FuncType>),

    UnaryOpExpr(Box<UnaryOpExpr>),
    BinaryOpExpr(Box<BinaryOpExpr>),
    ApplyExpr(Box<ApplyExpr>),

    Select(Box<Select>),
    Pipe(Box<Pipe>),

    IdentItem(Box<IdentItem>),

    LetDecl(Box<LetDecl>),
    VarDecl(Box<VarDecl>),
    TypeAliasDecl(Box<TypeAliasDecl>),
}

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub items: Vec<Item>,
}
