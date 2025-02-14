use crate::parser::detail::{MutOp, Statement};
use std::mem;

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
            Self::PushFront(l, r) => Self::PopFront(l.clone(), r.clone()),
            Self::PushBack(l, r) => Self::PopBack(l.clone(), r.clone()),
            Self::PopFront(l, r) => Self::PushFront(l.clone(), r.clone()),
            Self::PopBack(l, r) => Self::PushFront(l.clone(), r.clone()),
            Self::IndexedSwap(x, l, r) => Self::IndexedSwap(x.clone(), l.clone(), r.clone()),
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
            Self::For(_, _, _) | Self::IfThenElse(_, _, _) => unreachable!(),
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

pub fn concat<'a, T, I, F>(list: I, delim: &str, mut converter: F) -> String
where
    T: 'a,
    I: IntoIterator<Item = &'a T>,
    F: FnMut(&T) -> String,
{
    let mut buf = String::new();
    let mut d = "";
    for x in list {
        buf += mem::replace(&mut d, delim);
        buf += &converter(x);
    }
    buf
}
