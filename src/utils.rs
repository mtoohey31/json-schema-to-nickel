use std::rc::Rc;

use nickel_lang::{
    identifier::Ident,
    mk_app, mk_fun,
    term::{array::Array, make, ArrayAttrs, BinaryOp, RichTerm, Term, UnaryOp},
    types::{TypeF, Types},
};
use schemars::schema::SingleOrVec;

pub(super) fn is_single<T>(single_or_vec: &SingleOrVec<T>) -> bool {
    match single_or_vec {
        SingleOrVec::Single(_) => true,
        SingleOrVec::Vec(v) => v.len() == 1,
    }
}

pub(super) fn unwrap_single<T>(single_or_vec: &SingleOrVec<T>) -> &T {
    match single_or_vec {
        SingleOrVec::Single(s) => &*s,
        SingleOrVec::Vec(v) => &v[0],
    }
}

pub(super) fn all_of_types(subcontracts: &[RichTerm]) -> Types {
    Types(TypeF::Flat(mk_app!(
        mk_fun!(
            "contracts",
            "label",
            "value",
            mk_app!(
                make::op1(
                    UnaryOp::StaticAccess(Ident::new("foldl")),
                    make::var("array")
                ),
                mk_fun!(
                    "v",
                    "c",
                    mk_app!(
                        make::op1(
                            UnaryOp::StaticAccess(Ident::new("apply")),
                            make::var("contract")
                        ),
                        make::var("c"),
                        make::var("label"),
                        make::var("v")
                    )
                ),
                make::var("value"),
                make::var("contracts")
            )
        ),
        RichTerm::from(Term::Array(
            Array::new(Rc::from(subcontracts.clone())),
            ArrayAttrs::default()
        ))
    )))
}

pub(super) fn any_of_types(subpredicates: &[RichTerm]) -> Types {
    Types(TypeF::Flat(any_of_contract(subpredicates)))
}

pub(super) fn any_of_contract(subpredicates: &[RichTerm]) -> RichTerm {
    mk_app!(
        make::op1(
            UnaryOp::StaticAccess(Ident::new("from_predicate")),
            make::var("contract")
        ),
        any_of_predicate(subpredicates)
    )
}

pub(super) fn any_of_predicate(subpredicates: &[RichTerm]) -> RichTerm {
    let any_of_fun = mk_fun!(
        "predicates",
        "value",
        mk_app!(
            make::op1(
                UnaryOp::StaticAccess(Ident::new("foldl")),
                make::var("array")
            ),
            mk_fun!(
                "current",
                "predicate",
                mk_app!(
                    make::op1(UnaryOp::BoolOr(), make::var("predicate")),
                    mk_app!(make::var("predicate"), make::var("value"))
                )
            )
        )
    );
    mk_app!(
        any_of_fun,
        RichTerm::from(Term::Array(
            Array::new(Rc::from(subpredicates.clone())),
            ArrayAttrs::default()
        ))
    )
}
