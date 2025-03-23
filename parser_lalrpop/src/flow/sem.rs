// Copyright 2025 Jelly Terra <jellyterra@symboltics.com>
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0
// that can be found in the LICENSE file and https://mozilla.org/MPL/2.0/.

use crate::flow::ast;
use crate::flow::ast::Item;
use crate::flow::sem::SemanticError::UnexpectedNode;
use paracell_parser_sem::sem;
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum SemanticError<'a> {
    #[error("unexpected node")]
    UnexpectedNode { have: &'a Item }
}

pub trait ToSemantic<T> {
    fn to_semantic(&self) -> Result<T, SemanticError>;
}

macro_rules! def_semantic {
    ($self: ident : $ast: ty => $sem: ty $body: block) => {
        impl ToSemantic<$sem> for $ast {
            fn to_semantic($self: &Self) -> Result<$sem, SemanticError> {
                Ok($body)
            }
        }
    };
}

def_semantic! { self: ast::Nat => sem::Nat {
    sem::Nat {
        val: self.val,
    }
}}

def_semantic! { self: ast::RecordType => sem::RecordType {
    sem::RecordType {
        fields: self.fields.iter().map(|field| {
            Ok(sem::Field {
                ident: field.ident.lit.clone(),
                ty: field.item.expect_semantic_type()?,
            })
        }).collect::<Result<Vec<_>, _>>()?,
    }
}}

def_semantic! { self: ast::UnionType => sem::UnionType {
    sem::UnionType {
        variants: self.variants.iter().map(|variant| {
            Ok(sem::Variant {
                ident: variant.ident.lit.clone(),
                ty: variant.item.expect_semantic_type()?,
            })
        }).collect::<Result<Vec<_>, _>>()?,
    }
}}

def_semantic! { self: ast::FuncType => sem::FuncType {
    sem::FuncType {
        params: self.param_tuple.expect_semantic_func_tuple()?,
        result: self.result_ty.expect_semantic_type()?,
    }
}}

def_semantic! { self: ast::Select => sem::Select {
    sem::Select {
        expr: self.expr.expect_semantic_expr()?,
        ident: self.ident.lit.clone(),
    }
}}

def_semantic! { self: ast::Pipe => sem::Pipe {
    sem::Pipe {
        from: self.from.expect_semantic_expr()?,
        to: self.to.expect_semantic_expr()?,
    }
}}

def_semantic! { self: ast::Block => sem::Block {
    sem::Block {
        stmts: self.elems.iter().map(Item::expect_semantic_stmt).collect::<Result<Vec<_>, _>>()?,
    }
}}

def_semantic! { self: ast::Func => sem::Func {
    sem::Func {
        ty: self.ty.to_semantic()?,
        block: self.block.to_semantic()?,
    }
}}

def_semantic! { self: ast::RecordExpr => sem::RecordExpr {
    sem::RecordExpr {
        fields: self.fields.iter().map(|field| {
            Ok(sem::FieldFill {
                ident: field.ident.lit.clone(),
                expr: field.item.expect_semantic_expr()?,
            })
        }).collect::<Result<Vec<_>, _>>()?,
    }
}}

def_semantic! { self: ast::ApplyExpr => sem::ApplyExpr {
    sem::ApplyExpr {
        func: self.func.expect_semantic_expr()?,
        params: self.params.expect_semantic_func_param_tuple()?,
    }
}}

def_semantic! { self: ast::UnaryOpExpr => sem::ApplyExpr {
    sem::ApplyExpr {
        func: sem::Expr::Ident(self.op.to_literal().to_string()),
        params: sem::RecordExpr {
            fields: vec![sem::FieldFill { ident: 0.to_string(), expr: self.expr.expect_semantic_expr()? }],
        },
    }
}}

def_semantic! { self: ast::BinaryOpExpr => sem::ApplyExpr {
    sem::ApplyExpr {
        func: sem::Expr::Ident(self.op.to_literal().to_string()),
        params: sem::RecordExpr {
            fields: vec![
                sem::FieldFill { ident: 0.to_string(), expr: self.left.expect_semantic_expr()? },
                sem::FieldFill { ident: 1.to_string(), expr: self.right.expect_semantic_expr()? },
            ],
        },
    }
}}

def_semantic! { self: ast::Case => sem::Case {
    sem::Case {
        pattern: self.pattern.expect_semantic_expr()?,
        expr: self.expr.expect_semantic_expr()?,
    }
}}

def_semantic! { self: ast::Match => sem::Match {
    sem::Match {
        cases: self.cases.iter().map(ast::Case::to_semantic).collect::<Result<Vec<_>, _>>()?,
    }
}}

def_semantic! { self: ast::LetDecl => sem::LetDecl {
    sem::LetDecl {
        ident: self.ident.lit.clone(),
        expr: self.expr.expect_semantic_expr()?,
    }
}}

def_semantic! { self: ast::VarDecl => sem::VarDecl {
    sem::VarDecl {
        ident: self.ident.lit.clone(),
        expr: self.expr.expect_semantic_expr()?,
    }
}}

def_semantic! { self: ast::TypeAliasDecl => sem::TypeAliasDecl {
    sem::TypeAliasDecl {
        ident: self.ident.lit.clone(),
        ty: self.ty.expect_semantic_type()?,
    }
}}

def_semantic! { self: ast::SourceFile => sem::SourceFile {
    sem::SourceFile {
        decls: self.items.iter().map(Item::expect_semantic_decl).collect::<Result<Vec<_>, _>>()?,
    }
}}

impl ast::Tuple {
    pub fn expect_semantic_type_tuple(&self) -> Result<sem::RecordType, SemanticError> {
        Ok(sem::RecordType {
            fields: self.elems.iter().enumerate().map(|(i, field)| {
                Ok(sem::Field {
                    ident: i.to_string(),
                    ty: field.expect_semantic_type()?,
                })
            }).collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn expect_semantic_func_tuple(&self) -> Result<sem::RecordType, SemanticError> {
        Ok(sem::RecordType {
            fields: self.elems.iter().map(|field| {
                match field {
                    Item::IdentItem(field) => {
                        Ok(sem::Field {
                            ident: field.ident.lit.clone(),
                            ty: field.item.expect_semantic_type()?,
                        })
                    }
                    _ => Err(UnexpectedNode { have: field }),
                }
            }).collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn expect_semantic_expr_tuple(&self) -> Result<sem::RecordExpr, SemanticError> {
        Ok(sem::RecordExpr {
            fields: self.elems.iter().enumerate().map(|(i, field)| {
                Ok(sem::FieldFill {
                    ident: i.to_string(),
                    expr: field.expect_semantic_expr()?,
                })
            }).collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn expect_semantic_field_fill_tuple(&self) -> Result<sem::RecordExpr, SemanticError> {
        Ok(sem::RecordExpr {
            fields: self.elems.iter().map(|field| {
                match field {
                    Item::IdentItem(field) => {
                        Ok(sem::FieldFill {
                            ident: field.ident.lit.clone(),
                            expr: field.item.expect_semantic_expr()?,
                        })
                    }
                    _ => Err(UnexpectedNode { have: field }),
                }
            }).collect::<Result<Vec<_>, _>>()?,
        })
    }

    pub fn expect_semantic_func_param_tuple(&self) -> Result<sem::RecordExpr, SemanticError> {
        if self.elems.len() == 0 {
            Ok(sem::RecordExpr { fields: vec![] })
        } else {
            match self.elems[0] {
                Item::IdentItem(_) => self.expect_semantic_field_fill_tuple(),
                _ => self.expect_semantic_expr_tuple(),
            }
        }
    }
}

impl Item {
    pub fn expect_semantic_type(&self) -> Result<sem::Type, SemanticError> {
        Ok(match self {
            Item::Ident(v) => sem::Type::Ident(v.lit.to_string()),
            Item::Tuple(v) => sem::Type::Record(Box::from(v.expect_semantic_type_tuple()?)),
            Item::RecordType(v) => sem::Type::Record(Box::from(v.to_semantic()?)),
            Item::UnionType(v) => sem::Type::Union(Box::from(v.to_semantic()?)),
            Item::FuncType(v) => sem::Type::Func(Box::from(v.to_semantic()?)),

            Item::Nat(_)
            | Item::Block(_)
            | Item::Func(_)
            | Item::Match(_)
            | Item::TypeTuple(_)
            | Item::UnaryOpExpr(_)
            | Item::BinaryOpExpr(_)
            | Item::ApplyExpr(_)
            | Item::LetDecl(_)
            | Item::VarDecl(_)
            | Item::TypeAliasDecl(_)
            | Item::Select(_)
            | Item::Pipe(_)
            | Item::IdentItem(_) => return Err(UnexpectedNode { have: self }),
        })
    }

    pub fn expect_semantic_expr(&self) -> Result<sem::Expr, SemanticError> {
        Ok(match self {
            Item::Nat(v) => sem::Expr::Nat(v.to_semantic()?),
            Item::Ident(v) => sem::Expr::Ident(v.lit.to_string()),
            Item::Tuple(v) => sem::Expr::Record(Box::from(v.expect_semantic_func_param_tuple()?)),
            Item::Block(v) => sem::Expr::Block(Box::from(v.to_semantic()?)),
            Item::Func(v) => sem::Expr::Func(Box::from(v.to_semantic()?)),
            Item::Match(v) => sem::Expr::Match(Box::from(v.to_semantic()?)),
            Item::UnaryOpExpr(v) => sem::Expr::Apply(Box::from(v.to_semantic()?)),
            Item::BinaryOpExpr(v) => sem::Expr::Apply(Box::from(v.to_semantic()?)),
            Item::ApplyExpr(v) => sem::Expr::Apply(Box::from(v.to_semantic()?)),
            Item::Select(v) => sem::Expr::Select(Box::from(v.to_semantic()?)),
            Item::Pipe(v) => sem::Expr::Pipe(Box::from(v.to_semantic()?)),

            Item::TypeTuple(_)
            | Item::RecordType(_)
            | Item::UnionType(_)
            | Item::FuncType(_)
            | Item::LetDecl(_)
            | Item::VarDecl(_)
            | Item::TypeAliasDecl(_)
            | Item::IdentItem(_) => return Err(UnexpectedNode { have: self }),
        })
    }

    pub fn expect_semantic_decl(&self) -> Result<sem::Decl, SemanticError> {
        Ok(match self {
            Item::LetDecl(v) => sem::Decl::Let(v.to_semantic()?),
            Item::VarDecl(v) => sem::Decl::Var(v.to_semantic()?),
            Item::TypeAliasDecl(v) => sem::Decl::TypeAlias(v.to_semantic()?),

            Item::Nat(_)
            | Item::Ident(_)
            | Item::Tuple(_)
            | Item::Block(_)
            | Item::Func(_)
            | Item::Match(_)
            | Item::TypeTuple(_)
            | Item::RecordType(_)
            | Item::UnionType(_)
            | Item::FuncType(_)
            | Item::UnaryOpExpr(_)
            | Item::BinaryOpExpr(_)
            | Item::ApplyExpr(_)
            | Item::Select(_)
            | Item::Pipe(_)
            | Item::IdentItem(_) => return Err(UnexpectedNode { have: self }),
        })
    }

    pub fn expect_semantic_stmt(&self) -> Result<sem::Stmt, SemanticError> {
        Ok(match self {
            Item::Nat(_)
            | Item::Ident(_)
            | Item::Tuple(_)
            | Item::Block(_)
            | Item::Func(_)
            | Item::Match(_)
            | Item::UnaryOpExpr(_)
            | Item::BinaryOpExpr(_)
            | Item::ApplyExpr(_)
            | Item::Select(_)
            | Item::Pipe(_) => sem::Stmt::Expr(self.expect_semantic_expr()?),

            Item::LetDecl(_)
            | Item::VarDecl(_)
            | Item::TypeAliasDecl(_) => sem::Stmt::Decl(self.expect_semantic_decl()?),

            Item::TypeTuple(_)
            | Item::RecordType(_)
            | Item::UnionType(_)
            | Item::FuncType(_)
            | Item::IdentItem(_) => return Err(UnexpectedNode { have: self }),
        })
    }
}
