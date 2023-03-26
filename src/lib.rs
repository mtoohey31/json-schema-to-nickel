mod utils;

use std::rc::Rc;

use nickel_lang::{
    identifier::Ident,
    mk_app, mk_fun,
    term::{array::Array, make, ArrayAttrs, BinaryOp, RichTerm, Term, UnaryOp},
    types::{RecordRows, RecordRowsF, TypeF, Types},
};
use schemars::schema::{InstanceType, NumberValidation, Schema, SchemaObject, SingleOrVec};
use utils::{all_of_contract, is_single, unwrap_single};

pub fn schema_to_types(schema: Schema) -> Types {
    let schema = match schema {
        Schema::Bool(true) => return Types(TypeF::Dyn),
        Schema::Bool(false) => {
            return Types(TypeF::Flat(mk_fun!(
                "label",
                "value",
                mk_app!(
                    make::op1(
                        UnaryOp::StaticAccess(Ident::new("blame_with")),
                        make::var("contract")
                    ),
                    make::string("Never contract evaluated"),
                    make::var("label")
                )
            )))
        }
        Schema::Object(s) => s,
    };

    // Easy case, single type, nothing else
    if let SchemaObject {
        metadata: _,
        instance_type: Some(ref instance_type),
        format: _,
        enum_values: None,
        const_value: None,
        subschemas: None,
        number: None,
        string: None,
        array: None,
        object: None,
        reference: None,
        extensions: _,
    } = schema
    {
        if is_single(&instance_type) {
            return instance_type_to_types(*unwrap_single(&instance_type));
        };
    };

    if let SchemaObject {
        metadata: _,
        instance_type: Some(ref instance_type),
        format: _,
        enum_values: None,
        const_value: None,
        subschemas: None,
        number: None,
        string: None,
        array: None,
        object: None,
        reference: None,
        extensions: _,
    } = schema
    {
        if is_single(&instance_type) {
            return instance_type_to_types(*unwrap_single(&instance_type));
        };
    };

    let SchemaObject {
        metadata: _,
        instance_type,
        format: _,
        enum_values,
        const_value,
        subschemas: None,
        number,
        string,
        array,
        object,
        reference: None,
        extensions: _,
    } = &schema else {
        todo!()
    };

    let mut contracts = Vec::new();

    if let Some(instance_type) = instance_type {
        match instance_type {
            SingleOrVec::Single(s) => {}
            SingleOrVec::Vec(v) => {}
        }
    }

    Types(TypeF::Flat(mk_app!(
        all_of_contract(),
        Term::Array(Array::new(Rc::from(contracts)), ArrayAttrs::new())
    )))
}

fn instance_type_to_types(instance: InstanceType) -> Types {
    match instance {
        InstanceType::Number => Types(TypeF::Num),
        InstanceType::Boolean => Types(TypeF::Bool),
        InstanceType::String => Types(TypeF::Str),
        InstanceType::Integer => Types(TypeF::Flat(make::op1(
            UnaryOp::StaticAccess(Ident::new("Int")),
            make::var("num"),
        ))),
        InstanceType::Null => Types(TypeF::Flat(mk_fun!(
            "label",
            "value",
            mk_app!(
                make::op1(
                    UnaryOp::Ite(),
                    make::op2(BinaryOp::Eq(), make::var("value"), Term::Null)
                ),
                make::var("value"),
                mk_app!(
                    make::op1(
                        UnaryOp::StaticAccess(Ident::new("blame_with")),
                        make::var("contract"),
                    ),
                    make::string("expected null"),
                    make::var("label")
                )
            )
        ))),
        InstanceType::Object => Types(TypeF::Record(RecordRows(RecordRowsF::TailDyn))),
        InstanceType::Array => Types(TypeF::Array(Box::new(Types(TypeF::Dyn)))),
    }
}

fn number_validation_to_contracts(
    NumberValidation {
        multiple_of,
        maximum,
        exclusive_maximum,
        minimum,
        exclusive_minimum,
    }: NumberValidation,
) -> Vec<RichTerm> {
    let mut contracts = Vec::new();

    if let Some(multiple) = multiple_of {
        contracts.push(mk_fun!(
            "label",
            "value",
            mk_app!(
                make::op1(
                    UnaryOp::Ite(),
                    make::op2(
                        BinaryOp::Eq(),
                        make::op2(BinaryOp::Modulo(), make::var("value"), Term::Num(multiple)),
                        Term::Num(0.0),
                    )
                ),
                make::var("value"),
                mk_app!(
                    make::op1(
                        UnaryOp::StaticAccess(Ident::new("blame_with")),
                        make::var("contract"),
                    ),
                    make::string(format!("expected multiple of {}", multiple)),
                    make::var("label")
                )
            )
        ))
    };

    if let Some(maximum) = maximum {
        contracts.push(mk_fun!(
            "label",
            "value",
            mk_app!(
                make::op1(
                    UnaryOp::Ite(),
                    make::op2(BinaryOp::LessOrEq(), make::var("value"), Term::Num(maximum)),
                ),
                make::var("value"),
                mk_app!(
                    make::op1(
                        UnaryOp::StaticAccess(Ident::new("blame_with")),
                        make::var("contract"),
                    ),
                    make::string(format!("expected at most {}", maximum)),
                    make::var("label")
                )
            )
        ))
    };

    if let Some(exclusive_maximum) = exclusive_maximum {
        contracts.push(mk_fun!(
            "label",
            "value",
            mk_app!(
                make::op1(
                    UnaryOp::Ite(),
                    make::op2(
                        BinaryOp::LessThan(),
                        make::var("value"),
                        Term::Num(exclusive_maximum)
                    ),
                ),
                make::var("value"),
                mk_app!(
                    make::op1(
                        UnaryOp::StaticAccess(Ident::new("blame_with")),
                        make::var("contract"),
                    ),
                    make::string(format!("expected less than {}", exclusive_maximum)),
                    make::var("label")
                )
            )
        ))
    };

    if let Some(minimum) = minimum {
        contracts.push(mk_fun!(
            "label",
            "value",
            mk_app!(
                make::op1(
                    UnaryOp::Ite(),
                    make::op2(
                        BinaryOp::GreaterOrEq(),
                        make::var("value"),
                        Term::Num(minimum)
                    ),
                ),
                make::var("value"),
                mk_app!(
                    make::op1(
                        UnaryOp::StaticAccess(Ident::new("blame_with")),
                        make::var("contract"),
                    ),
                    make::string(format!("expected at least {}", minimum)),
                    make::var("label")
                )
            )
        ))
    };

    if let Some(exclusive_minimum) = exclusive_minimum {
        contracts.push(mk_fun!(
            "label",
            "value",
            mk_app!(
                make::op1(
                    UnaryOp::Ite(),
                    make::op2(
                        BinaryOp::GreaterThan(),
                        make::var("value"),
                        Term::Num(exclusive_minimum)
                    ),
                ),
                make::var("value"),
                mk_app!(
                    make::op1(
                        UnaryOp::StaticAccess(Ident::new("blame_with")),
                        make::var("contract"),
                    ),
                    make::string(format!("expected more than {}", exclusive_minimum)),
                    make::var("label")
                )
            )
        ))
    };

    contracts
}
