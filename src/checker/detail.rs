use crate::checker::Mutables;
use crate::parser::detail::{Expr, Statement, Variable};
use crate::parser::r#for::For;

pub trait CheckMut {
    fn check_mut(&self, mutables: &mut Mutables);
}

pub trait HasVariable {
    fn has_variable(&self, variable: &Variable) -> bool;
}

impl CheckMut for Expr {
    fn check_mut(&self, mutables: &mut Mutables) {
        match self {
            Self::Const(_) | Self::Nil => {}
            Self::Variable(x) | Self::Empty(x) | Self::Size(x) => x.check_mut(mutables),
            Self::Array(l) => {
                for e in l.as_ref() {
                    e.check_mut(mutables);
                }
            }
            Self::Indexed(x, i) => {
                x.check_mut(mutables);
                i.check_mut(mutables);
            }
            Self::BinOp(l, _, r) => {
                l.check_mut(mutables);
                r.check_mut(mutables);
            }
            Self::UnrOp(_, e) | Self::Wrapped(e) => e.check_mut(mutables),
        }
    }
}

impl CheckMut for Variable {
    fn check_mut(&self, mutables: &mut Mutables) {
        match mutables.get_mut(self) {
            None => {}
            Some(true) => panic!(
                "mutable variable `{}` cannot be used more than once in involution",
                self.0
            ),
            Some(x) => *x = true,
        }
    }
}

impl CheckMut for Statement {
    fn check_mut(&self, mutables: &mut Mutables) {
        match self {
            Self::Mut(x, _, e) => {
                x.check_mut(mutables);
                e.check_mut(mutables);
            }
            Self::IndexedMut(x, l, _, r) | Self::IndexedSwap(x, l, r) => {
                x.check_mut(mutables);
                l.check_mut(mutables);
                r.check_mut(mutables);
            }
            Self::Call(_, args) | Self::Uncall(_, args) => {
                for arg in args {
                    arg.check_mut(mutables);
                }
            }
            Self::Skip | Self::Print(_) => {}
            Self::IfThenElse(e, s_l, s_r) => {
                e.check_mut(mutables);
                let mut cloned = mutables.clone();
                s_l.check_mut(&mut cloned);
                s_r.check_mut(mutables);
                for ((_, m), (_, c)) in mutables.iter_mut().zip(cloned) {
                    *m |= c;
                }
            }
            Self::For(For {
                vars,
                containers,
                statement,
            }) => {
                let cloned = mutables.clone();

                for (c, i) in containers {
                    c.check_mut(mutables);
                    if let Some(i) = i {
                        i.check_mut(mutables);
                    }
                }

                for (_, b) in mutables.iter_mut() {
                    *b = true;
                }

                for (vs, (c, _)) in vars.iter().zip(containers) {
                    if mutables.get(c).is_some() {
                        for v in vs {
                            mutables.insert(v.clone(), false);
                        }
                    }
                }

                statement.check_mut(mutables);

                *mutables = cloned;
            }
            Self::Sequence(l, r) => {
                l.check_mut(mutables);
                r.check_mut(mutables);
            }
            _ => unreachable!(),
        }
    }
}

impl HasVariable for Expr {
    fn has_variable(&self, variable: &Variable) -> bool {
        match self {
            Self::Const(_) | Self::Nil => false,
            Self::Variable(x) | Self::Empty(x) | Self::Size(x) => x == variable,
            Self::Array(l) => l.iter().any(|e| e.has_variable(variable)),
            Self::Indexed(x, i) => x == variable || i.has_variable(variable),
            Self::BinOp(l, _, r) => l.has_variable(variable) || r.has_variable(variable),
            Self::UnrOp(_, e) | Self::Wrapped(e) => e.has_variable(variable),
        }
    }
}
