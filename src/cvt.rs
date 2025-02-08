mod detail;

use crate::parser::detail::{
    BinOp, Expr, MainProc, MutOp, Proc, ProcId, Program, Statement, Type, TypedVariable, UnrOp,
    Variable,
};
use detail::{concat, Flip};
use std::mem;

const INDENT_WIDTH: usize = 4;

fn indent(depth: usize) -> String {
    (0..INDENT_WIDTH * depth).map(|_| ' ').collect()
}

pub trait Cvt {
    fn cvt(&self) -> String;
}

trait CvtInd {
    fn cvt_ind(&self, depth: usize) -> String;
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
            Self::Swap => format!("swap({l}, {r});"),
        }
    }
}

impl Cvt for Type {
    fn cvt(&self) -> String {
        match self {
            Self::Int => "Int".to_string(),
            Self::Array(n) => format!("Array<{n}>"),
            Self::List => "List".to_string(),
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
        let mut buf = "#include \"prelude.hpp\"\n\n".to_string();

        for proc in procs {
            buf += &format!("{}\n", proc.cvt_sig());
        }

        buf += &format!("\n{}", main.cvt());

        for proc in procs {
            buf += &format!("\n{}", proc.cvt());
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
        let Self(decls, statement, invl) = self;
        let mut buf = "int main() {\n".to_string();
        let spaces = indent(1);
        for (t_x, e) in decls {
            let rhs = e
                .as_ref()
                .map(|x| format!(" = {}", x.cvt()))
                .unwrap_or("{}".to_string());
            buf += &format!("{spaces}{}{};\n", t_x.cvt(), rhs);
        }
        buf += &format!("\n{}", statement.cvt_ind(1));
        buf += &format!("\n{}", invl.cvt_ind(1));
        buf += &format!("\n{}\n", statement.flip().cvt_ind(1));
        for (TypedVariable(_, var), _) in decls {
            let name = var.0.as_ref();
            buf += &format!("{spaces}print(\"{name}\", {name});\n");
        }
        buf += "}\n";
        buf
    }
}

impl Cvt for Proc {
    fn cvt(&self) -> String {
        let mut buf = String::new();

        match self {
            Self::Inj(name, args, statement) => {
                buf += &format!("void {}_fwd(", name.cvt());
                buf += &concat(args, ", ", |arg| arg.cvt_ref());
                buf += &format!(
                    ") {{\n{}}}\n\nvoid {}_rev(",
                    statement.cvt_ind(1),
                    name.cvt()
                );
                buf += &concat(args, ", ", |arg| arg.cvt_ref());
                buf += &format!(") {{\n{}}}\n", statement.flip().cvt_ind(1));
            }
            Self::Invl(name, args, statement, invl) => {
                let body = concat(args, ", ", |arg| arg.cvt_ref())
                    + &format!(
                        ") {{\n{}\n{}\n{}}}\n",
                        statement.cvt_ind(1),
                        invl.cvt_ind(1),
                        statement.flip().cvt_ind(1),
                    );

                buf += &format!("void {}_fwd(", name.cvt());
                buf += &body;
                buf += &format!("\nvoid {}_rev(", name.cvt());
                buf += &body;
            }
            Self::Mat(name, mat) => {
                let args: Vec<_> = (0..mat.size)
                    .map(|i| Box::new(move |c| format!("{c}{i}")) as Box<dyn Fn(char) -> String>)
                    .collect();

                let mut body = concat(&args, ", ", |arg| format!("Int& {}", arg('v'))) + ") {\n";
                let spaces = indent(1);

                for arg in &args {
                    body += &format!("{spaces}Int {} = {};\n", arg('c'), arg('v'));
                }

                for (i, arg) in args.iter().enumerate() {
                    body += &format!("{spaces}{} = ", arg('v'));

                    let mut delim = "";
                    for (j, var) in args.iter().enumerate() {
                        body += mem::replace(&mut delim, " + ");
                        body += &format!("{} * {}", mat.get(i, j), var('c'));
                    }
                    body += ";\n";
                }
                body += "}\n";

                buf += &format!("void {}_fwd(", name.cvt());
                buf += &body;
                buf += &format!("\nvoid {}_rev(", name.cvt());
                buf += &body;
            }
        }

        buf
    }
}

impl CvtSig for Proc {
    fn cvt_sig(&self) -> String {
        let hello = match self {
            Self::Inj(_, args, _) | Self::Invl(_, args, _, _) => {
                concat(args, ", ", |arg| arg.cvt_ref())
            }
            Self::Mat(_, mat) => {
                let args: Vec<String> = (0..mat.size).map(|i| format!("v{i}")).collect();
                concat(&args, ", ", |arg| format!("Int& {arg}"))
            }
        };

        let (Self::Inj(name, _, _) | Self::Invl(name, _, _, _) | Self::Mat(name, _)) = self;
        let mut buf = format!("void {}_fwd(", name.cvt());
        buf += &hello;
        buf += &format!(");\nvoid {}_rev(", name.cvt());
        buf += &hello;
        buf += ");";
        buf
    }
}

impl CvtInd for Statement {
    fn cvt_ind(&self, depth: usize) -> String {
        let spaces = indent(depth);
        let more_spaces = indent(depth + 1);

        match self {
            Self::Mut(x, op, e) => spaces + &op.cvt_mut_op(&x.cvt(), &e.cvt()) + "\n",
            Self::IndexedMut(x, i, op, e) => {
                spaces + &op.cvt_mut_op(&format!("{}[{}]", x.cvt(), i.cvt()), &e.cvt()) + "\n"
            }
            Self::IfThenElseFi(e_l, s_l, s_r, e_r) => format!(
                "{spaces}if ({0}) {{\n{1}{more_spaces}assert({3});\n{spaces}}} else {{\n{2}{more_spaces}assert(!({3}));\n{spaces}}}\n",
                e_l.cvt(),
                s_l.cvt_ind(depth + 1),
                s_r.cvt_ind(depth + 1),
                e_r.cvt()
            ),
            Self::FromDoLoopUntil(e_l, s_l, s_r, e_r) => format!(
                "{spaces}assert({0});\n{1}{spaces}while (!({3})) {{\n{2}{more_spaces}assert(!({0}));\n{4}{spaces}}}\n",
                e_l.cvt(),
                s_l.cvt_ind(depth),
                s_r.cvt_ind(depth + 1),
                e_r.cvt(),
                s_l.cvt_ind(depth + 1),
            ),
            Self::Push(l, r) => format!("{spaces}{0}.push_front({1});\n{spaces}{1} = 0;\n", r.cvt(), l.cvt()),
            Self::Pop(l, r) => format!(
                "{spaces}assert({1} == 0);\n{spaces}{1} = {0}.front();\n{spaces}{0}.pop_front();\n",
                r.cvt(),
                l.cvt()
            ),
            Self::LocalDelocal(tx_l, e_l, s, tx_r, e_r) => format!(
                "{spaces}{{\n{more_spaces}{} = {};\n{}{more_spaces}assert({} == {});\n{spaces}}}\n",
                tx_l.cvt(),
                e_l.cvt(),
                s.cvt_ind(depth + 1),
                tx_r.1.cvt(),
                e_r.cvt()
            ),
            either @ (Self::Call(q, args) | Self::Uncall(q, args)) => {
                let postfix = match either {
                    Self::Call(_, _) => "fwd",
                    Self::Uncall(_, _) => "rev",
                    _ => unreachable!(),
                };
                let mut buf = format!("{spaces}{}_{}(", q.cvt(), postfix);
                buf += &concat(args, ", ", |arg| arg.cvt());
                buf += ");\n";
                buf
            }
            Self::Skip => String::new(),
            Self::Print(x) => format!("{spaces}print(\"{0}\", {0});\n", x.0),
            Self::Sequence(l, r) => format!("{}{}", l.cvt_ind(depth), r.cvt_ind(depth)),
        }
    }
}

impl Cvt for Expr {
    fn cvt(&self) -> String {
        match self {
            Self::Const(x) => x.to_string(),
            Self::Variable(x) => x.cvt(),
            Self::Array(x) => format!("{{{}}}", concat(x.as_ref(), ", ", |item| item.cvt())),
            Self::Indexed(x, e) => format!("{}[{}]", x.cvt(), e.cvt()),
            Self::BinOp(l, op, r) => format!("{} {} {}", l.cvt(), op.cvt(), r.cvt()),
            Self::UnrOp(op, x) => format!("{}{}", op.cvt(), x.cvt()),
            Self::Empty(x) => format!("{}.empty()", x.cvt()),
            Self::Top(x) => format!("{}.front()", x.cvt()),
            Self::Nil => "List{}".to_string(),
            Self::Size(x) => format!("{}.size()", x.cvt()),
            Self::Wrapped(x) => format!("({})", x.cvt()),
        }
    }
}
