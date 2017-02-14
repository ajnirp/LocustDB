use value::ValueType;
use std:rc::Rc;

#[derive(Debug)]
pub struct Query {
    pub select: Vec<String>,
    pub filter: Condition,
}

#[derive(Debug)]
pub enum Condition {
    True,
    False,
    Column(String),
    Func(FuncType, Rc<Condition>, Rc<Condition>),
    Const(Rc<ValueType>),
}

#[derive(Debug)]
pub enum FuncType {
    Equals,
    LT,
    GT,
    And,
    Or
}

/*
fn run(query: Query, source: Vec<Vec<ValueType>>) -> Vec<Vec<ValueType>> {
    source.iter().filter(|record| {

    })
}*/


fn eval(record: fn(String) -> Option<Arc<ValueType>>, condition: &Condition) -> Arc<ValueType> {
    use self::Condition::*;
    use self::ValueType::*;
    match condition {
        &True => Bool(true),
        &False => Bool(false),
        &Func(ref functype, ref exp1, ref exp2) =>
            match (functype, eval(record, exp1), eval(record, exp2)) {
                (FuncType::Equals, v1,          v2)          => Bool(v1 == v2),
                (FuncType::And,    Bool(b1),    Bool(b2))    => Bool(b1 && b2),
                (FuncType::Or,     Bool(b1),    Bool(b2))    => Bool(b1 || b2),
                (FuncType::LT,     Integer(i1), Integer(i2)) => Bool(i1 < i2),
                (FuncType::LT,     Float(f1),   Float(f2))   => Bool(f1 < f2),
                (FuncType::GT,     Integer(i1), Integer(i2)) => Bool(i1 > i2),
                (FuncType::GT,     Float(f1),   Float(f2))   => Bool(f1 > f2),
                (functype, v1, v2) => panic!("Type error: function {:?} not defined for values {:?} and {:?}", functype, v1, v2),
            },
        &Column(col_name) => record.iter().find(|col_entry| col_entry.0 == col_name).unwrap().1,
        &Const(value) => value,
    }
}