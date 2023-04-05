use nom::{branch::alt, character::streaming::space0, combinator::map, sequence::tuple, IResult};

use super::atom::*;

#[derive(Debug)]
pub enum Operation {
    N(f64),
    V(Variable),
    NAddN {
        left: f64,
        right: f64,
    },
    NAddV {
        left: f64,
        right: Variable,
    },
    NAddO {
        left: f64,
        right: Box<Operation>,
    },
    VAddN {
        left: Variable,
        right: f64,
    },
    VAddV {
        left: Variable,
        right: Variable,
    },
    VAddO {
        left: Variable,
        right: Box<Operation>,
    },
    OAddN {
        left: Box<Operation>,
        right: f64,
    },
    OAddV {
        left: Box<Operation>,
        right: Variable,
    },
    OAddO {
        left: Box<Operation>,
        right: Box<Operation>,
    },
    NSubN {
        left: f64,
        right: f64,
    },
    NSubV {
        left: f64,
        right: Variable,
    },
    NSubO {
        left: f64,
        right: Box<Operation>,
    },
    VSubN {
        left: Variable,
        right: f64,
    },
    VSubV {
        left: Variable,
        right: Variable,
    },
    VSubO {
        left: Variable,
        right: Box<Operation>,
    },
    OSubN {
        left: Box<Operation>,
        right: f64,
    },
    OSubV {
        left: Box<Operation>,
        right: Variable,
    },
    OSubO {
        left: Box<Operation>,
        right: Box<Operation>,
    },
    NMulN {
        left: f64,
        right: f64,
    },
    NMulV {
        left: f64,
        right: Variable,
    },
    NMulO {
        left: f64,
        right: Box<Operation>,
    },
    VMulN {
        left: Variable,
        right: f64,
    },
    VMulV {
        left: Variable,
        right: Variable,
    },
    VMulO {
        left: Variable,
        right: Box<Operation>,
    },
    OMulN {
        left: Box<Operation>,
        right: f64,
    },
    OMulV {
        left: Box<Operation>,
        right: Variable,
    },
    OMulO {
        left: Box<Operation>,
        right: Box<Operation>,
    },
    NDivN {
        left: f64,
        right: f64,
    },
    NDivV {
        left: f64,
        right: Variable,
    },
    NDivO {
        left: f64,
        right: Box<Operation>,
    },
    VDivN {
        left: Variable,
        right: f64,
    },
    VDivV {
        left: Variable,
        right: Variable,
    },
    VDivO {
        left: Variable,
        right: Box<Operation>,
    },
    ODivN {
        left: Box<Operation>,
        right: f64,
    },
    ODivV {
        left: Box<Operation>,
        right: Variable,
    },
    ODivO {
        left: Box<Operation>,
        right: Box<Operation>,
    },
    NModN {
        left: f64,
        right: f64,
    },
    NModV {
        left: f64,
        right: Variable,
    },
    NModO {
        left: f64,
        right: Box<Operation>,
    },
    VModN {
        left: Variable,
        right: f64,
    },
    VModV {
        left: Variable,
        right: Variable,
    },
    VModO {
        left: Variable,
        right: Box<Operation>,
    },
    OModN {
        left: Box<Operation>,
        right: f64,
    },
    OModV {
        left: Box<Operation>,
        right: Variable,
    },
    OModO {
        left: Box<Operation>,
        right: Box<Operation>,
    },
    NPowN {
        left: f64,
        right: f64,
    },
    NPowV {
        left: f64,
        right: Variable,
    },
    NPowO {
        left: f64,
        right: Box<Operation>,
    },
    VPowN {
        left: Variable,
        right: f64,
    },
    VPowV {
        left: Variable,
        right: Variable,
    },
    VPowO {
        left: Variable,
        right: Box<Operation>,
    },
    OPowN {
        left: Box<Operation>,
        right: f64,
    },
    OPowV {
        left: Box<Operation>,
        right: Variable,
    },
    OPowO {
        left: Box<Operation>,
        right: Box<Operation>,
    },
}

pub fn n(input: &str) -> IResult<&str, Box<Operation>> {
    map(tuple((p_left, number, p_right)), |(_, num, _)| {
        Box::new(Operation::N(num))
    })(input)
}

pub fn v(input: &str) -> IResult<&str, Box<Operation>> {
    map(tuple((p_left, variable, p_right)), |(_, var, _)| {
        Box::new(Operation::V(var))
    })(input)
}

macro_rules! bi_operation {
    ($fname:ident, $left_func:ident, $oper_func:ident, $right_func:ident, $left_type:ty, $oper_type:ident, $right_type:ty, $oper:ident) => {
        fn $fname(input: &str) -> IResult<&str, Box<Operation>> {
            map(
                tuple((
                    p_left,
                    space0,
                    $left_func,
                    space0,
                    $oper_func,
                    space0,
                    $right_func,
                    space0,
                    p_right,
                )),
                |(_, _, left, _, _, _, right, _, _): (
                    PLeft,
                    &str,
                    $left_type,
                    &str,
                    $oper_type,
                    &str,
                    $right_type,
                    &str,
                    PRight,
                )| { Box::new(Operation::$oper { left, right }) },
            )(input)
        }
    };
}

bi_operation!(n_add_n, number, add, number, f64, Add, f64, NAddN);
bi_operation!(n_add_v, number, add, variable, f64, Add, Variable, NAddV);
bi_operation!(
    n_add_o,
    number,
    add,
    operation,
    f64,
    Add,
    Box<Operation>,
    NAddO
);
bi_operation!(v_add_n, variable, add, number, Variable, Add, f64, VAddN);
bi_operation!(v_add_v, variable, add, variable, Variable, Add, Variable, VAddV);
bi_operation!(
    v_add_o,
    variable,
    add,
    operation,
    Variable,
    Add,
    Box<Operation>,
    VAddO
);
bi_operation!(
    o_add_n,
    operation,
    add,
    number,
    Box<Operation>,
    Add,
    f64,
    OAddN
);
bi_operation!(
    o_add_v,
    operation,
    add,
    variable,
    Box<Operation>,
    Add,
    Variable,
    OAddV
);
bi_operation!(
    o_add_o,
    operation,
    add,
    operation,
    Box<Operation>,
    Add,
    Box<Operation>,
    OAddO
);
bi_operation!(n_sub_n, number, sub, number, f64, Sub, f64, NSubN);
bi_operation!(n_sub_v, number, sub, variable, f64, Sub, Variable, NSubV);
bi_operation!(
    n_sub_o,
    number,
    sub,
    operation,
    f64,
    Sub,
    Box<Operation>,
    NSubO
);
bi_operation!(v_sub_n, variable, sub, number, Variable, Sub, f64, VSubN);
bi_operation!(v_sub_v, variable, sub, variable, Variable, Sub, Variable, VSubV);
bi_operation!(
    v_sub_o,
    variable,
    sub,
    operation,
    Variable,
    Sub,
    Box<Operation>,
    VSubO
);
bi_operation!(
    o_sub_n,
    operation,
    sub,
    number,
    Box<Operation>,
    Sub,
    f64,
    OSubN
);
bi_operation!(
    o_sub_v,
    operation,
    sub,
    variable,
    Box<Operation>,
    Sub,
    Variable,
    OSubV
);
bi_operation!(
    o_sub_o,
    operation,
    sub,
    operation,
    Box<Operation>,
    Sub,
    Box<Operation>,
    OSubO
);
bi_operation!(n_mul_n, number, mul, number, f64, Mul, f64, NMulN);
bi_operation!(n_mul_v, number, mul, variable, f64, Mul, Variable, NMulV);
bi_operation!(
    n_mul_o,
    number,
    mul,
    operation,
    f64,
    Mul,
    Box<Operation>,
    NMulO
);
bi_operation!(v_mul_n, variable, mul, number, Variable, Mul, f64, VMulN);
bi_operation!(v_mul_v, variable, mul, variable, Variable, Mul, Variable, VMulV);
bi_operation!(
    v_mul_o,
    variable,
    mul,
    operation,
    Variable,
    Mul,
    Box<Operation>,
    VMulO
);
bi_operation!(
    o_mul_n,
    operation,
    mul,
    number,
    Box<Operation>,
    Mul,
    f64,
    OMulN
);
bi_operation!(
    o_mul_v,
    operation,
    mul,
    variable,
    Box<Operation>,
    Mul,
    Variable,
    OMulV
);
bi_operation!(
    o_mul_o,
    operation,
    mul,
    operation,
    Box<Operation>,
    Mul,
    Box<Operation>,
    OMulO
);
bi_operation!(n_div_n, number, div, number, f64, Div, f64, NDivN);
bi_operation!(n_div_v, number, div, variable, f64, Div, Variable, NDivV);
bi_operation!(
    n_div_o,
    number,
    div,
    operation,
    f64,
    Div,
    Box<Operation>,
    NDivO
);
bi_operation!(v_div_n, variable, div, number, Variable, Div, f64, VDivN);
bi_operation!(v_div_v, variable, div, variable, Variable, Div, Variable, VDivV);
bi_operation!(
    v_div_o,
    variable,
    div,
    operation,
    Variable,
    Div,
    Box<Operation>,
    VDivO
);
bi_operation!(
    o_div_n,
    operation,
    div,
    number,
    Box<Operation>,
    Div,
    f64,
    ODivN
);
bi_operation!(
    o_div_v,
    operation,
    div,
    variable,
    Box<Operation>,
    Div,
    Variable,
    ODivV
);
bi_operation!(
    o_div_o,
    operation,
    div,
    operation,
    Box<Operation>,
    Div,
    Box<Operation>,
    ODivO
);
bi_operation!(n_mod_n, number, module, number, f64, Mod, f64, NModN);
bi_operation!(n_mod_v, number, module, variable, f64, Mod, Variable, NModV);
bi_operation!(
    n_mod_o,
    number,
    module,
    operation,
    f64,
    Mod,
    Box<Operation>,
    NModO
);
bi_operation!(v_mod_n, variable, module, number, Variable, Mod, f64, VModN);
bi_operation!(v_mod_v, variable, module, variable, Variable, Mod, Variable, VModV);
bi_operation!(
    v_mod_o,
    variable,
    module,
    operation,
    Variable,
    Mod,
    Box<Operation>,
    VModO
);
bi_operation!(
    o_mod_n,
    operation,
    module,
    number,
    Box<Operation>,
    Mod,
    f64,
    OModN
);
bi_operation!(
    o_mod_v,
    operation,
    module,
    variable,
    Box<Operation>,
    Mod,
    Variable,
    OModV
);
bi_operation!(
    o_mod_o,
    operation,
    module,
    operation,
    Box<Operation>,
    Mod,
    Box<Operation>,
    OModO
);
bi_operation!(n_pow_n, number, pow, number, f64, Pow, f64, NPowN);
bi_operation!(n_pow_v, number, pow, variable, f64, Pow, Variable, NPowV);
bi_operation!(
    n_pow_o,
    number,
    pow,
    operation,
    f64,
    Pow,
    Box<Operation>,
    NPowO
);
bi_operation!(v_pow_n, variable, pow, number, Variable, Pow, f64, VPowN);
bi_operation!(v_pow_v, variable, pow, variable, Variable, Pow, Variable, VPowV);
bi_operation!(
    v_pow_o,
    variable,
    pow,
    operation,
    Variable,
    Pow,
    Box<Operation>,
    VPowO
);
bi_operation!(
    o_pow_n,
    operation,
    pow,
    number,
    Box<Operation>,
    Pow,
    f64,
    OPowN
);
bi_operation!(
    o_pow_v,
    operation,
    pow,
    variable,
    Box<Operation>,
    Pow,
    Variable,
    OPowV
);
bi_operation!(
    o_pow_o,
    operation,
    pow,
    operation,
    Box<Operation>,
    Pow,
    Box<Operation>,
    OPowO
);

pub fn operation(input: &str) -> IResult<&str, Box<Operation>> {
    alt((
        n,
        v,
        alt((
            n_add_n, n_add_v, n_add_o, v_add_n, v_add_v, v_add_o, o_add_n, o_add_v, o_add_o,
        )),
        alt((
            n_sub_n, n_sub_v, n_sub_o, v_sub_n, v_sub_v, v_sub_o, o_sub_n, o_sub_v, o_sub_o,
        )),
        alt((
            n_mul_n, n_mul_v, n_mul_o, v_mul_n, v_mul_v, v_mul_o, o_mul_n, o_mul_v, o_mul_o,
        )),
        alt((
            n_div_n, n_div_v, n_div_o, v_div_n, v_div_v, v_div_o, o_div_n, o_div_v, o_div_o,
        )),
        alt((
            n_mod_n, n_mod_v, n_mod_o, v_mod_n, v_mod_v, v_mod_o, o_mod_n, o_mod_v, o_mod_o,
        )),
        alt((
            n_pow_n, n_pow_v, n_pow_o, v_pow_n, v_pow_v, v_pow_o, o_pow_n, o_pow_v, o_pow_o,
        )),
    ))(input)
}
