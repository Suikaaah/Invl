use crate::parser::detail::{
    Expr, MainProc, Proc, ProcId, Program, Statement, TypedVariable, Variable,
};
use std::collections::BTreeMap;

#[derive(Debug)]
enum ProcType {
    Inj,
    Invl,
}

#[derive(Debug)]
pub struct Checker {
    proc_types: BTreeMap<ProcId, ProcType>,
}

type Mutables = BTreeMap<Variable, bool>;

impl Checker {
    pub fn check(program: &Program) {
        Self::new(program).check_program(program);
    }

    fn new(program: &Program) -> Self {
        let Program(_, procs) = program;
        let mut proc_types = BTreeMap::<ProcId, ProcType>::new();
        for proc in procs {
            if let (id, Some(_)) = match proc {
                Proc::Inj(id, _, _) => (id, proc_types.insert(id.clone(), ProcType::Inj)),
                Proc::Invl(id, _, _, _) | Proc::Mat(id, _) => {
                    (id, proc_types.insert(id.clone(), ProcType::Invl))
                }
            } {
                panic!("colliding function names: {}", id.0);
            }
        }
        Self { proc_types }
    }

    fn check_program(&self, program: &Program) {
        let Program(main, procs) = program;
        self.check_main(main);
        for proc in procs {
            self.check_proc(proc);
        }
    }

    fn assert_invl(&self, id: &ProcId) {
        match self.proc_types.get(id) {
            Some(ProcType::Inj) => panic!("expected invl, found inj"),
            Some(ProcType::Invl) => {}
            None => panic!("undefined function found: {:?}", id.0),
        }
    }

    fn ban_inj_call(&self, invl: &Statement) {
        match invl {
            Statement::Call(id, _) | Statement::Uncall(id, _) => self.assert_invl(id),
            Statement::For(_, _, s) => self.ban_inj_call(s),
            Statement::IfThenElse(_, s_l, s_r) | Statement::Sequence(s_l, s_r) => {
                self.ban_inj_call(s_l);
                self.ban_inj_call(s_r);
            }
            _ => {}
        }
    }

    fn check_expr(mutables: &mut Mutables, expr: &Expr) {
        match expr {
            Expr::Const(_) | Expr::Nil => {}
            Expr::Variable(x) | Expr::Empty(x) | Expr::Top(x) | Expr::Size(x) => {
                Self::check_variable(mutables, x)
            }
            Expr::Array(l) => {
                for e in l.as_ref() {
                    Self::check_expr(mutables, e);
                }
            }
            Expr::Indexed(x, i) => {
                Self::check_variable(mutables, x);
                Self::check_expr(mutables, i);
            }
            Expr::BinOp(l, _, r) => {
                Self::check_expr(mutables, l);
                Self::check_expr(mutables, r);
            }
            Expr::UnrOp(_, e) | Expr::Wrapped(e) => Self::check_expr(mutables, e),
        }
    }

    fn check_variable(mutables: &mut Mutables, variable: &Variable) {
        match mutables.get_mut(variable) {
            None => {}
            Some(true) => panic!(
                "mutable variable `{variable:?}` cannot be used more than once in involution"
            ),
            Some(x) => *x = true,
        }
    }

    fn check_invl(mutables: &mut Mutables, invl: &Statement) {
        match invl {
            Statement::Mut(x, _, e) => {
                Self::check_variable(mutables, x);
                Self::check_expr(mutables, e);
            }
            Statement::IndexedMut(x, i, _, e) => {
                Self::check_variable(mutables, x);
                Self::check_expr(mutables, i);
                Self::check_expr(mutables, e);
            }
            Statement::Call(_, es) | Statement::Uncall(_, es) => {
                for e in es {
                    Self::check_expr(mutables, e);
                }
            }
            Statement::Skip | Statement::Print(_) => {}
            Statement::IfThenElse(e, s_l, s_r) => {
                Self::check_expr(mutables, e);
                let mut cloned = mutables.clone();
                Self::check_invl(&mut cloned, s_l);
                Self::check_invl(mutables, s_r);
                for ((_, m), (_, c)) in mutables.iter_mut().zip(cloned) {
                    *m |= c;
                }
            }
            Statement::For(xs_l, xs_r, s) => {
                let cloned = mutables.clone();

                for (l, r) in xs_l.iter().zip(xs_r) {
                    Self::check_variable(mutables, r);
                    if let Some(_) = mutables.get(r) {
                        mutables.insert(l.clone(), false);
                    }
                }
                Self::check_invl(mutables, s);

                *mutables = cloned;
            }
            Statement::Sequence(l, r) => {
                Self::check_invl(mutables, l);
                Self::check_invl(mutables, r);
            }
            _ => unreachable!(),
        }
    }

    fn mutables<'a, I>(variables: I) -> Mutables
    where
        I: IntoIterator<Item = &'a TypedVariable>,
    {
        let mut mutables = Mutables::new();

        for TypedVariable(t, x) in variables {
            if !t.r#const {
                mutables.insert(x.clone(), false);
            }
        }

        mutables
    }

    fn check_main(&self, main: &MainProc) {
        let MainProc(decls, _, invl) = main;
        self.ban_inj_call(invl);
        let mut mutables = Self::mutables(decls.iter().map(|(t_x, _)| t_x));
        Self::check_invl(&mut mutables, invl);
    }

    fn check_proc(&self, proc: &Proc) {
        if let Proc::Invl(_, params, _, invl) = proc {
            self.ban_inj_call(invl);
            let mut mutables = Self::mutables(params);
            Self::check_invl(&mut mutables, invl)
        }
    }
}
