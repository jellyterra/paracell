// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use std::cell::RefCell;
use std::collections::HashMap;
use paracell_util_struct::map::OrderedHashMap;

#[derive(Clone, Debug)]
pub struct Field<'a> {
    pub ident: String,
    pub ty: Type<'a>,
}

#[derive(Clone, Debug)]
pub struct RecordType<'a> {
    pub fields: Vec<Field<'a>>,
    pub names: HashMap<String, usize>,
}

#[derive(Clone, Debug)]
pub struct Variant<'a> {
    pub ident: String,
    pub ty: Type<'a>,
}

#[derive(Clone, Debug)]
pub struct UnionType<'a> {
    pub variants: Vec<Variant<'a>>,
    pub names: HashMap<String, usize>,
}

#[derive(Clone, Debug)]
pub struct NatType {}

#[derive(Clone, Debug)]
pub enum PrimitiveType {
    Nat(NatType),
}

#[derive(Clone, Debug)]
pub struct FuncType<'a> {
    pub params: RecordType<'a>,
    pub results: Type<'a>,
}

#[derive(Clone, Debug)]
pub enum Type<'a> {
    Primitive(PrimitiveType),
    Record(RefCell<RecordType<'a>>),
    Union(RefCell<UnionType<'a>>),
}

#[derive(Clone, Debug)]
pub struct NatExpr {
    pub val: u128,
}

#[derive(Clone, Debug)]
pub struct Case<'a> {
    pub pattern: Expr<'a>,
    pub expr: Scope<'a>,
}

#[derive(Clone, Debug)]
pub struct Match<'a> {
    pub cases: Vec<Case<'a>>,
}

#[derive(Clone, Debug)]
pub enum Expr<'a> {
    Nat(NatExpr),
    Match(Match<'a>),
}

#[derive(Clone, Debug)]
pub struct LetDecl<'a> {
    pub ident: String,
    pub expr: Expr<'a>,
}

#[derive(Clone, Debug)]
pub struct VarDecl<'a> {
    pub ident: String,
    pub expr: Expr<'a>,
}

#[derive(Clone, Debug)]
pub struct TypeAliasDecl<'a> {
    pub ident: String,
    pub ty: Type<'a>,
}

#[derive(Clone, Debug)]
pub enum Decl<'a> {
    Let(LetDecl<'a>),
    Var(VarDecl<'a>),
    TypeAlias(TypeAliasDecl<'a>),
}

#[derive(Clone, Debug)]
pub struct Scope<'a> {
    pub decls: OrderedHashMap<&'a str, &'a Decl<'a>>,
    pub expr: Expr<'a>,
}
