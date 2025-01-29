mod detail;

use crate::parser::detail::{
    BinOp, Expr, MainProc, MutOp, Proc, ProcId, Program, Statement, Type, TypedVariable, UnrOp,
    Variable,
};
use detail::Reverse;
use std::mem;

pub trait Cvt {
    fn cvt(&self) -> String;
}

trait CvtRef {
    fn cvt_ref(&self) -> String;
}

trait CvtSig {
    fn cvt_sig(&self) -> String;
}

trait CvtMutOp {
    fn cvt_mut_op(&self, l: &str, r: &str) -> String;
}

impl CvtMutOp for MutOp {
    fn cvt_mut_op(&self, l: &str, r: &str) -> String {
        match self {
            Self::Add => format!("{l} += {r};"),
            Self::Sub => format!("{l} -= {r};"),
            Self::Xor => format!("{l} ^= {r};"),
            Self::Swap => format!("std::swap({l}, {r});"),
        }
    }
}

impl Cvt for Type {
    fn cvt(&self) -> String {
        match self {
            Self::Int => "int".to_string(),
            Self::Array(n) => format!("std::array<int, {n}>"),
            Self::List => "std::deque<int>".to_string(),
        }
    }
}

impl Cvt for BinOp {
    fn cvt(&self) -> String {
        match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Xor => "^",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Remainder => "%",
            Self::BitwiseAnd => "&",
            Self::BitwiseOr => "|",
            Self::LogicalAnd => "&&",
            Self::LogicalOr => "||",
            Self::LessThan => "<",
            Self::GreaterThan => ">",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::LessEqual => "<=",
            Self::GreaterEqual => ">=",
        }
        .to_string()
    }
}

impl Cvt for UnrOp {
    fn cvt(&self) -> String {
        match self {
            Self::Negative => "-",
            Self::Not => "!",
        }
        .to_string()
    }
}

impl Cvt for Variable {
    fn cvt(&self) -> String {
        self.0.to_string()
    }
}

impl Cvt for ProcId {
    fn cvt(&self) -> String {
        self.0.to_string()
    }
}

impl Cvt for Program {
    fn cvt(&self) -> String {
        let Self(main, procs) = self;
        let mut buf = "#include\"prelude.hpp\"\n\n".to_string();

        for proc in procs {
            buf += &format!("{}\n", proc.cvt_sig());
        }

        buf += &format!("\n{}\n", main.cvt());

        for proc in procs {
            buf += &format!("{}\n", proc.cvt());
        }

        buf
    }
}

impl Cvt for TypedVariable {
    fn cvt(&self) -> String {
        let Self(t, x) = self;
        format!("{} {}", t.cvt(), x.cvt())
    }
}

impl CvtRef for TypedVariable {
    fn cvt_ref(&self) -> String {
        let Self(t, x) = self;
        format!("{}& {}", t.cvt(), x.cvt())
    }
}

impl Cvt for MainProc {
    fn cvt(&self) -> String {
        let Self(decls, statement) = self;
        let mut buf = "int main() {\n".to_string();
        for decl in decls {
            buf += &format!("{}{{}};\n", decl.cvt());
        }
        buf += &format!("\n{}\n", statement.cvt());

        for decl in decls {
            buf += &format!("\nstd::cout << \"{0}: \" << {0} << '\\n';", decl.1.cvt());
        }

        buf += "\n}\n";
        buf
    }
}

impl Cvt for Proc {
    fn cvt(&self) -> String {
        let Self(name, args, statement) = self;
        let mut buf = format!("void {}_fwd(", name.cvt());
        let mut delim = "";
        for arg in args {
            buf += mem::replace(&mut delim, ", ");
            buf += &arg.cvt_ref();
        }
        buf += &format!(") {{\n{}\n}}\n\nvoid {}_rev(", statement.cvt(), name.cvt());
        delim = "";
        for arg in args {
            buf += mem::replace(&mut delim, ", ");
            buf += &arg.cvt_ref();
        }
        buf += &format!(") {{\n{}\n}}\n", statement.reverse().cvt());
        buf
    }
}

impl CvtSig for Proc {
    fn cvt_sig(&self) -> String {
        let Self(name, args, _) = self;
        let mut buf = format!("void {}_fwd(", name.cvt());
        let mut delim = "";
        for arg in args {
            buf += mem::replace(&mut delim, ", ");
            buf += &arg.cvt_ref();
        }
        buf += &format!(");\nvoid {}_rev(", name.cvt());
        delim = "";
        for arg in args {
            buf += mem::replace(&mut delim, ", ");
            buf += &arg.cvt_ref();
        }
        buf += ");";
        buf
    }
}

impl Cvt for Statement {
    fn cvt(&self) -> String {
        match self {
            Self::Mut(x, op, e) => op.cvt_mut_op(&x.cvt(), &e.cvt()),
            Self::IndexedMut(x, i, op, e) => {
                op.cvt_mut_op(&format!("{}[{}]", x.cvt(), i.cvt()), &e.cvt())
            }
            Self::IfThenElseFi(e_l, s_l, s_r, e_r) => format!(
                "if ({0}) {{\n{1}\nassert({3});\n}} else {{\n{2}\nassert(!({3}));\n}}",
                e_l.cvt(),
                s_l.cvt(),
                s_r.cvt(),
                e_r.cvt()
            ),
            Self::FromDoLoopUntil(e_l, s_l, s_r, e_r) => format!(
                "assert({0});\n{1}\nwhile (!({3})) {{\n{2}\nassert(!({0}));\n{1}\n}}",
                e_l.cvt(),
                s_l.cvt(),
                s_r.cvt(),
                e_r.cvt()
            ),
            Self::Push(l, r) => format!("{0}.push_front({1});\n{1} = 0;", r.cvt(), l.cvt()),
            Self::Pop(l, r) => format!(
                "assert({1} == 0);\n{1} = {0}.front();\n{0}.pop_front();",
                r.cvt(),
                l.cvt()
            ),
            Self::LocalDelocal(tx_l, e_l, s, tx_r, e_r) => format!(
                "{{\n{} = {};\n{}\nassert({} == {});\n}}",
                tx_l.cvt(),
                e_l.cvt(),
                s.cvt(),
                tx_r.1.cvt(),
                e_r.cvt()
            ),
            either @ (Self::Call(q, args) | Self::Uncall(q, args)) => {
                let postfix = match either {
                    Self::Call(_, _) => "fwd",
                    Self::Uncall(_, _) => "rev",
                    _ => unreachable!(),
                };
                let mut buf = format!("{}_{}(", q.cvt(), postfix);
                let mut delim = "";
                for arg in args {
                    buf += mem::replace(&mut delim, ", ");
                    buf += &arg.cvt();
                }
                buf += ");";
                buf
            }
            Self::Skip => String::new(),
            Self::Sequence(l, r) => format!("{}\n{}", l.cvt(), r.cvt()),
        }
    }
}

impl Cvt for Expr {
    fn cvt(&self) -> String {
        match self {
            Self::Const(x) => x.to_string(),
            Self::Variable(x) => x.cvt(),
            Self::Indexed(x, e) => format!("{}[{}]", x.cvt(), e.cvt()),
            Self::BinOp(l, op, r) => format!("{} {} {}", l.cvt(), op.cvt(), r.cvt()),
            Self::UnrOp(op, x) => format!("{}{}", op.cvt(), x.cvt()),
            Self::Empty(x) => format!("{}.empty()", x.cvt()),
            Self::Top(x) => format!("{}.front()", x.cvt()),
            Self::Nil => "std::deque<int>{}".to_string(),
            Self::Size(x) => format!("{}.size()", x.cvt()),
            Self::Wrapped(x) => format!("({})", x.cvt()),
        }
    }
}
