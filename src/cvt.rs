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

trait CvtIndCelled {
    fn cvt_ind_celled(&self, depth: usize) -> String;
}

trait CvtCelled {
    fn cvt_celled(&self) -> String;
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
        buf += &format!("\n{}\n", statement.cvt_ind(1));
        for (TypedVariable(_, var), _) in decls {
            buf += &format!("{spaces}auto {0}_ = make_cell({0});\n", var.0);
        }
        buf += &format!("{spaces}Cells cells(");
        buf += &concat(decls, ", ", |(TypedVariable(_, var), _)| {
            format!("{}_", var.0)
        });
        buf += &format!(");\n\n{}", invl.cvt_ind_celled(1));
        buf += &format!("\n{}\n", statement.flip().cvt_ind(1));
        for (TypedVariable(_, var), _) in decls {
            buf += &format!("{spaces}print(\"{0}\", {0});\n", var.0);
        }
        buf += "}\n";
        buf
    }
}

impl Cvt for Proc {
    fn cvt(&self) -> String {
        let mut buf = String::new();
        let spaces = indent(1);

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
                let mut body = concat(args, ", ", |arg| arg.cvt_ref())
                    + &format!(") {{\n{}\n", statement.cvt_ind(1));
                for TypedVariable(_, var) in args {
                    body += &format!("{spaces}auto {0}_ = make_cell({0});\n", var.0);
                }
                body += &format!("{spaces}Cells cells(");
                body += &concat(args, ", ", |TypedVariable(_, var)| format!("{}_", var.0));
                body += &format!(");\n\n{}", invl.cvt_ind_celled(1));
                body += &format!("\n{}}}\n", statement.flip().cvt_ind(1));

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
            Self::For(_, _, _) => unreachable!(),
            Self::Sequence(l, r) => format!("{}{}", l.cvt_ind(depth), r.cvt_ind(depth)),
        }
    }
}

impl CvtIndCelled for Statement {
    fn cvt_ind_celled(&self, depth: usize) -> String {
        let spaces = indent(depth);
        let more_spaces = indent(depth + 1);
        let even_more_spaces = indent(depth + 2);

        match self {
            Self::Mut(x, op, e) => {
                format!(
                    "{spaces}{}\n{spaces}cells.update();\n",
                    op.cvt_mut_op(&x.cvt_celled(), &e.cvt_celled())
                )
            }
            Self::IndexedMut(x, i, op, e) => {
                format!(
                    "{spaces}{}\n{spaces}cells.update();\n",
                    op.cvt_mut_op(
                        &format!("{}_[{}]", x.cvt(), i.cvt_celled()),
                        &e.cvt_celled(),
                    )
                )
            }
            either @ (Self::Call(q, args) | Self::Uncall(q, args)) => {
                let postfix = match either {
                    Self::Call(_, _) => "fwd",
                    Self::Uncall(_, _) => "rev",
                    _ => unreachable!(),
                };
                let mut buf = format!("{spaces}{}_{}(", q.cvt(), postfix);
                buf += &concat(args, ", ", |arg| arg.cvt_celled());
                buf += &format!(");\n{spaces}cells.update();\n");
                buf
            }
            Self::Skip => String::new(),
            Self::Print(x) => format!("{spaces}print(\"{0}\", {0});\n", x.0),
            Self::Sequence(l, r) => {
                format!("{}{}", l.cvt_ind_celled(depth), r.cvt_ind_celled(depth))
            }
            Self::For(x, rep, s) => {
                format!(
                    "{spaces}{{\n{more_spaces}Cell {0}_;\n{more_spaces}cells.push({0}_);\n",
                    x.cvt()
                ) + &format!(
                    "{more_spaces}for (Int i = 0; i < {}; ++i) {{\n",
                    rep.cvt_celled()
                ) + &format!(
                    "{even_more_spaces}{}_ = make_cell(i); {};\n{even_more_spaces}cells.update();\n\n",
                    x.cvt(), x.cvt_celled()
                ) + &s.cvt_ind_celled(depth + 2)
                    + &format!("{more_spaces}}}\n{more_spaces}cells.pop();\n{spaces}}}\n")
            }
            _ => unreachable!(),
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

impl CvtCelled for Variable {
    fn cvt_celled(&self) -> String {
        format!("(*{}_)", self.cvt())
    }
}

impl CvtCelled for Expr {
    fn cvt_celled(&self) -> String {
        match self {
            Self::Const(x) => x.to_string(),
            Self::Variable(x) => x.cvt_celled(),
            Self::Indexed(x, e) => format!("{}_[{}]", x.cvt(), e.cvt_celled()),
            Self::BinOp(l, op, r) => format!("{} {} {}", l.cvt_celled(), op.cvt(), r.cvt_celled()),
            Self::UnrOp(op, x) => format!("{}{}", op.cvt(), x.cvt_celled()),
            Self::Wrapped(x) => format!("({})", x.cvt_celled()),
            Self::Empty(x) => format!("{}_->empty()", x.cvt()),
            Self::Top(x) => format!("{}_->front()", x.cvt()),
            Self::Size(x) => format!("{}_->size()", x.cvt()),
            _ => unreachable!(),
        }
    }
}
