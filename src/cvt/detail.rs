use crate::parser::detail::{MutOp, Statement};

pub trait Flip {
    fn flip(&self) -> Self;
}

impl Flip for Statement {
    fn flip(&self) -> Self {
        match self {
            Self::Mut(x, op, e) => Self::Mut(x.clone(), op.flip(), e.clone()),
            Self::IndexedMut(x, i, op, e) => {
                Self::IndexedMut(x.clone(), i.clone(), op.flip(), e.clone())
            }
            Self::IfThenElseFi(e_l, s_l, s_r, e_r) => Self::IfThenElseFi(
                e_r.clone(),
                Box::new(s_l.flip()),
                Box::new(s_r.flip()),
                e_l.clone(),
            ),
            Self::FromDoLoopUntil(e_l, s_l, s_r, e_r) => Self::FromDoLoopUntil(
                e_r.clone(),
                Box::new(s_l.flip()),
                Box::new(s_r.flip()),
                e_l.clone(),
            ),
            Self::Push(l, r) => Self::Pop(l.clone(), r.clone()),
            Self::Pop(l, r) => Self::Push(l.clone(), r.clone()),
            Self::LocalDelocal(tx_l, e_l, s, tx_r, e_r) => Self::LocalDelocal(
                tx_r.clone(),
                e_r.clone(),
                Box::new(s.flip()),
                tx_l.clone(),
                e_l.clone(),
            ),
            Self::Call(q, args) => Self::Uncall(q.clone(), args.clone()),
            Self::Uncall(q, args) => Self::Call(q.clone(), args.clone()),
            Self::Skip => Self::Skip,
            Self::Print(x) => Self::Print(x.clone()),
            Self::Sequence(l, r) => Self::Sequence(Box::new(r.flip()), Box::new(l.flip())),
        }
    }
}

impl Flip for MutOp {
    fn flip(&self) -> Self {
        match self {
            Self::Add => Self::Sub,
            Self::Sub => Self::Add,
            Self::Xor => Self::Xor,
            Self::Swap => Self::Swap,
        }
    }
}
