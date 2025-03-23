// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

// Atoms

use paracell_util_macro::AsVariant;

#[derive(Clone, Debug)]
pub struct Ident {
    pub lit: String,
}

#[derive(Clone, Debug)]
pub struct Nat {
    pub val: u128,
}

// Types

#[derive(Clone, Debug)]
pub struct Field {
    pub ident: String,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub struct RecordType {
    pub fields: Vec<Field>,
}

#[derive(Clone, Debug)]
pub struct Variant {
    pub ident: String,
    pub ty: Type,
}

#[derive(Clone, Debug)]
pub struct UnionType {
    pub variants: Vec<Variant>,
}

#[derive(Clone, Debug)]
pub struct FuncType {
    pub params: RecordType,
    pub result: Type,
}

#[derive(Clone, Debug, AsVariant)]
pub enum Type {
    Ident(String),
    Record(Box<RecordType>),
    Union(Box<UnionType>),
    Func(Box<FuncType>),
}

// Expressions

#[derive(Clone, Debug)]
pub struct Select {
    pub expr: Expr,
    pub ident: String,
}

#[derive(Clone, Debug)]
pub struct Pipe {
    pub from: Expr,
    pub to: Expr,
}

#[derive(Clone, Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub struct Func {
    pub ty: FuncType,
    pub block: Block,
}

#[derive(Clone, Debug)]
pub struct FieldFill {
    pub ident: String,
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct RecordExpr {
    pub fields: Vec<FieldFill>,
}

#[derive(Clone, Debug)]
pub struct ApplyExpr {
    pub func: Expr,
    pub params: RecordExpr,
}

#[derive(Clone, Debug)]
pub struct Case {
    pub pattern: Expr,
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct Match {
    pub cases: Vec<Case>,
}

#[derive(Clone, Debug, AsVariant)]
pub enum Expr {
    Nat(Nat),
    Ident(String),
    Block(Box<Block>),
    Func(Box<Func>),
    Record(Box<RecordExpr>),
    Apply(Box<ApplyExpr>),
    Match(Box<Match>),

    Select(Box<Select>),
    Pipe(Box<Pipe>),
}

// Declarations

#[derive(Clone, Debug)]
pub struct LetDecl {
    pub ident: String,
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct VarDecl {
    pub ident: String,
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct TypeAliasDecl {
    pub ident: String,
    pub ty: Type,
}

#[derive(Clone, Debug, AsVariant)]
pub enum Decl {
    Let(LetDecl),
    Var(VarDecl),
    TypeAlias(TypeAliasDecl),
}

// Statement

#[derive(Clone, Debug, AsVariant)]
pub enum Stmt {
    Decl(Decl),
    Expr(Expr),
}

#[derive(Clone, Debug)]
pub struct SourceFile {
    pub decls: Vec<Decl>,
}
