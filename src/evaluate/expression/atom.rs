use nom::{bytes::streaming::tag, combinator::map, number::streaming::double, IResult};

pub fn number(input: &str) -> IResult<&str, f64> {
    double(input)
}

macro_rules! operator {
    ($sname:ident, $fname:ident, $symbol:literal) => {
        #[derive(Debug)]
        pub struct $sname;
        pub fn $fname(input: &str) -> IResult<&str, $sname> {
            map(tag($symbol), |_| $sname)(input)
        }
    };
}

operator!(Variable, variable, "x");
operator!(Add, add, "+");
operator!(Sub, sub, "-");
operator!(Mul, mul, "*");
operator!(Div, div, "/");
operator!(Mod, module, "%");
operator!(Pow, pow, "^");
operator!(Log, log, "log");
operator!(PLeft, p_left, "(");
operator!(PRight, p_right, ")");
