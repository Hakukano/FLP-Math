use core::fmt;

use num_traits::{FromPrimitive, ToPrimitive};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

use self::{
    atom::Variable,
    operation::Operation::{self, *},
};

mod atom;
mod operation;

#[derive(Debug)]
struct BinaryOperation {
    left: Box<Evaluate>,
    right: Box<Evaluate>,
}

impl BinaryOperation {
    fn new(left: Evaluate, right: Evaluate) -> Self {
        Self {
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
enum Node {
    Add(BinaryOperation),
    Sub(BinaryOperation),
    Mul(BinaryOperation),
    Div(BinaryOperation),
    Mod(BinaryOperation),
    Pow(BinaryOperation),
    Log(BinaryOperation),
    Number(f64),
    Variable,
}

/// Evaluate a math expression
///
/// # Examples
///
/// ```
/// use flp_math::evaluate::Evaluate as Super;
/// use flp_math::evaluate::expression::Evaluate;
/// let evaluate = Evaluate::try_from_str("(6)").unwrap();
/// assert_eq!(Super::<u8, u8>::evaluate(&evaluate, 9), 6);
/// let evaluate = Evaluate::try_from_str("(x)").unwrap();
/// assert_eq!(Super::<u8, u8>::evaluate(&evaluate, 9), 9);
/// let evaluate = Evaluate::try_from_str("(((2 * (x ^2))-((6/ x) +(25 log 5)))%80)").unwrap();
/// assert_eq!(Super::<u8, u8>::evaluate(&evaluate, 7), 15);
/// ```
#[derive(Debug)]
pub struct Evaluate {
    node: Node,
}

impl Evaluate {
    pub fn try_from_str(s: &str) -> Result<Self, String> {
        let operation = operation::operation(s).map_err(|err| err.to_string())?.1;
        (*operation)
            .try_into()
            .map_err(|_| "Cannot parse expression".to_string())
    }
}

impl From<f64> for Evaluate {
    fn from(value: f64) -> Self {
        Self {
            node: Node::Number(value),
        }
    }
}

impl From<Variable> for Evaluate {
    fn from(_: Variable) -> Self {
        Self {
            node: Node::Variable,
        }
    }
}

impl From<Operation> for Evaluate {
    fn from(value: Operation) -> Self {
        match value {
            N(value) => value.into(),
            V(value) => value.into(),
            NAddN { left, right } => Self {
                node: Node::Add(BinaryOperation::new(left.into(), right.into())),
            },
            NAddV { left, right } => Self {
                node: Node::Add(BinaryOperation::new(left.into(), right.into())),
            },
            NAddO { left, right } => Self {
                node: Node::Add(BinaryOperation::new(left.into(), (*right).into())),
            },
            VAddN { left, right } => Self {
                node: Node::Add(BinaryOperation::new(left.into(), right.into())),
            },
            VAddV { left, right } => Self {
                node: Node::Add(BinaryOperation::new(left.into(), right.into())),
            },
            VAddO { left, right } => Self {
                node: Node::Add(BinaryOperation::new(left.into(), (*right).into())),
            },
            OAddN { left, right } => Self {
                node: Node::Add(BinaryOperation::new((*left).into(), right.into())),
            },
            OAddV { left, right } => Self {
                node: Node::Add(BinaryOperation::new((*left).into(), right.into())),
            },
            OAddO { left, right } => Self {
                node: Node::Add(BinaryOperation::new((*left).into(), (*right).into())),
            },
            NSubN { left, right } => Self {
                node: Node::Sub(BinaryOperation::new(left.into(), right.into())),
            },
            NSubV { left, right } => Self {
                node: Node::Sub(BinaryOperation::new(left.into(), right.into())),
            },
            NSubO { left, right } => Self {
                node: Node::Sub(BinaryOperation::new(left.into(), (*right).into())),
            },
            VSubN { left, right } => Self {
                node: Node::Sub(BinaryOperation::new(left.into(), right.into())),
            },
            VSubV { left, right } => Self {
                node: Node::Sub(BinaryOperation::new(left.into(), right.into())),
            },
            VSubO { left, right } => Self {
                node: Node::Sub(BinaryOperation::new(left.into(), (*right).into())),
            },
            OSubN { left, right } => Self {
                node: Node::Sub(BinaryOperation::new((*left).into(), right.into())),
            },
            OSubV { left, right } => Self {
                node: Node::Sub(BinaryOperation::new((*left).into(), right.into())),
            },
            OSubO { left, right } => Self {
                node: Node::Sub(BinaryOperation::new((*left).into(), (*right).into())),
            },
            NMulN { left, right } => Self {
                node: Node::Mul(BinaryOperation::new(left.into(), right.into())),
            },
            NMulV { left, right } => Self {
                node: Node::Mul(BinaryOperation::new(left.into(), right.into())),
            },
            NMulO { left, right } => Self {
                node: Node::Mul(BinaryOperation::new(left.into(), (*right).into())),
            },
            VMulN { left, right } => Self {
                node: Node::Mul(BinaryOperation::new(left.into(), right.into())),
            },
            VMulV { left, right } => Self {
                node: Node::Mul(BinaryOperation::new(left.into(), right.into())),
            },
            VMulO { left, right } => Self {
                node: Node::Mul(BinaryOperation::new(left.into(), (*right).into())),
            },
            OMulN { left, right } => Self {
                node: Node::Mul(BinaryOperation::new((*left).into(), right.into())),
            },
            OMulV { left, right } => Self {
                node: Node::Mul(BinaryOperation::new((*left).into(), right.into())),
            },
            OMulO { left, right } => Self {
                node: Node::Mul(BinaryOperation::new((*left).into(), (*right).into())),
            },
            NDivN { left, right } => Self {
                node: Node::Div(BinaryOperation::new(left.into(), right.into())),
            },
            NDivV { left, right } => Self {
                node: Node::Div(BinaryOperation::new(left.into(), right.into())),
            },
            NDivO { left, right } => Self {
                node: Node::Div(BinaryOperation::new(left.into(), (*right).into())),
            },
            VDivN { left, right } => Self {
                node: Node::Div(BinaryOperation::new(left.into(), right.into())),
            },
            VDivV { left, right } => Self {
                node: Node::Div(BinaryOperation::new(left.into(), right.into())),
            },
            VDivO { left, right } => Self {
                node: Node::Div(BinaryOperation::new(left.into(), (*right).into())),
            },
            ODivN { left, right } => Self {
                node: Node::Div(BinaryOperation::new((*left).into(), right.into())),
            },
            ODivV { left, right } => Self {
                node: Node::Div(BinaryOperation::new((*left).into(), right.into())),
            },
            ODivO { left, right } => Self {
                node: Node::Div(BinaryOperation::new((*left).into(), (*right).into())),
            },
            NModN { left, right } => Self {
                node: Node::Mod(BinaryOperation::new(left.into(), right.into())),
            },
            NModV { left, right } => Self {
                node: Node::Mod(BinaryOperation::new(left.into(), right.into())),
            },
            NModO { left, right } => Self {
                node: Node::Mod(BinaryOperation::new(left.into(), (*right).into())),
            },
            VModN { left, right } => Self {
                node: Node::Mod(BinaryOperation::new(left.into(), right.into())),
            },
            VModV { left, right } => Self {
                node: Node::Mod(BinaryOperation::new(left.into(), right.into())),
            },
            VModO { left, right } => Self {
                node: Node::Mod(BinaryOperation::new(left.into(), (*right).into())),
            },
            OModN { left, right } => Self {
                node: Node::Mod(BinaryOperation::new((*left).into(), right.into())),
            },
            OModV { left, right } => Self {
                node: Node::Mod(BinaryOperation::new((*left).into(), right.into())),
            },
            OModO { left, right } => Self {
                node: Node::Mod(BinaryOperation::new((*left).into(), (*right).into())),
            },
            NPowN { left, right } => Self {
                node: Node::Pow(BinaryOperation::new(left.into(), right.into())),
            },
            NPowV { left, right } => Self {
                node: Node::Pow(BinaryOperation::new(left.into(), right.into())),
            },
            NPowO { left, right } => Self {
                node: Node::Pow(BinaryOperation::new(left.into(), (*right).into())),
            },
            VPowN { left, right } => Self {
                node: Node::Pow(BinaryOperation::new(left.into(), right.into())),
            },
            VPowV { left, right } => Self {
                node: Node::Pow(BinaryOperation::new(left.into(), right.into())),
            },
            VPowO { left, right } => Self {
                node: Node::Pow(BinaryOperation::new(left.into(), (*right).into())),
            },
            OPowN { left, right } => Self {
                node: Node::Pow(BinaryOperation::new((*left).into(), right.into())),
            },
            OPowV { left, right } => Self {
                node: Node::Pow(BinaryOperation::new((*left).into(), right.into())),
            },
            OPowO { left, right } => Self {
                node: Node::Pow(BinaryOperation::new((*left).into(), (*right).into())),
            },
            NLogN { left, right } => Self {
                node: Node::Log(BinaryOperation::new(left.into(), right.into())),
            },
            NLogV { left, right } => Self {
                node: Node::Log(BinaryOperation::new(left.into(), right.into())),
            },
            NLogO { left, right } => Self {
                node: Node::Log(BinaryOperation::new(left.into(), (*right).into())),
            },
            VLogN { left, right } => Self {
                node: Node::Log(BinaryOperation::new(left.into(), right.into())),
            },
            VLogV { left, right } => Self {
                node: Node::Log(BinaryOperation::new(left.into(), right.into())),
            },
            VLogO { left, right } => Self {
                node: Node::Log(BinaryOperation::new(left.into(), (*right).into())),
            },
            OLogN { left, right } => Self {
                node: Node::Log(BinaryOperation::new((*left).into(), right.into())),
            },
            OLogV { left, right } => Self {
                node: Node::Log(BinaryOperation::new((*left).into(), right.into())),
            },
            OLogO { left, right } => Self {
                node: Node::Log(BinaryOperation::new((*left).into(), (*right).into())),
            },
        }
    }
}

impl ToString for Evaluate {
    fn to_string(&self) -> String {
        match &self.node {
            Node::Add(node) => {
                format!(
                    "({} + {})",
                    node.left.as_ref().to_string(),
                    node.right.as_ref().to_string()
                )
            }
            Node::Sub(node) => {
                format!(
                    "({} - {})",
                    node.left.as_ref().to_string(),
                    node.right.as_ref().to_string()
                )
            }
            Node::Mul(node) => {
                format!(
                    "({} * {})",
                    node.left.as_ref().to_string(),
                    node.right.as_ref().to_string()
                )
            }
            Node::Div(node) => {
                format!(
                    "({} / {})",
                    node.left.as_ref().to_string(),
                    node.right.as_ref().to_string()
                )
            }
            Node::Mod(node) => {
                format!(
                    "({} % {})",
                    node.left.as_ref().to_string(),
                    node.right.as_ref().to_string()
                )
            }
            Node::Pow(node) => {
                format!(
                    "({} ^ {})",
                    node.left.as_ref().to_string(),
                    node.right.as_ref().to_string()
                )
            }
            Node::Log(node) => {
                format!(
                    "({} log {})",
                    node.left.as_ref().to_string(),
                    node.right.as_ref().to_string()
                )
            }
            Node::Number(node) => node.to_string(),
            Node::Variable => "x".to_string(),
        }
    }
}

fn evaluate_recursive(variable: f64, evaluate: &Evaluate) -> f64 {
    match &evaluate.node {
        Node::Add(node) => {
            evaluate_recursive(variable, node.left.as_ref())
                + evaluate_recursive(variable, node.right.as_ref())
        }
        Node::Sub(node) => {
            evaluate_recursive(variable, node.left.as_ref())
                - evaluate_recursive(variable, node.right.as_ref())
        }
        Node::Mul(node) => {
            evaluate_recursive(variable, node.left.as_ref())
                * evaluate_recursive(variable, node.right.as_ref())
        }
        Node::Div(node) => {
            evaluate_recursive(variable, node.left.as_ref())
                / evaluate_recursive(variable, node.right.as_ref())
        }
        Node::Mod(node) => {
            evaluate_recursive(variable, node.left.as_ref())
                % evaluate_recursive(variable, node.right.as_ref())
        }
        Node::Pow(node) => evaluate_recursive(variable, node.left.as_ref())
            .powf(evaluate_recursive(variable, node.right.as_ref())),
        Node::Log(node) => evaluate_recursive(variable, node.left.as_ref())
            .log(evaluate_recursive(variable, node.right.as_ref())),
        Node::Number(node) => *node,
        Node::Variable => variable,
    }
}

impl<X, Y> super::Evaluate<X, Y> for Evaluate
where
    X: ToPrimitive,
    Y: FromPrimitive,
{
    fn evaluate(&self, x: X) -> Y {
        Y::from_f64(evaluate_recursive(
            x.to_f64().expect("Cannot convert X to f64"),
            self,
        ))
        .expect("Cannot convert f64 to Y")
    }
}

struct EvaluateVisitor;

impl<'de> Visitor<'de> for EvaluateVisitor {
    type Value = Evaluate;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a valid expression")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Evaluate::try_from_str(v).map_err(|err| de::Error::custom(err))
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Evaluate::try_from_str(v.as_str()).map_err(|err| de::Error::custom(err))
    }
}

impl<'de> Deserialize<'de> for Evaluate {
    fn deserialize<D>(deserializer: D) -> Result<Evaluate, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(EvaluateVisitor)
    }
}

impl Serialize for Evaluate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
