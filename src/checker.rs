mod detail;

use crate::parser::{
    detail::{MainProc, Proc, ProcId, Program, Statement, TypedVariable, Variable},
    r#for::For,
};
use detail::{CheckMut, HasVariable};
use std::collections::{BTreeMap, BTreeSet};

type Mutables = BTreeMap<Variable, bool>;

#[derive(Debug)]
enum ProcType {
    Inj,
    Invl,
}

#[derive(Debug)]
pub struct Checker {
    proc_types: BTreeMap<ProcId, ProcType>,
}

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
            Statement::For(For { statement, .. }) => self.ban_inj_call(statement),
            Statement::IfThenElse(_, s_l, s_r) | Statement::Sequence(s_l, s_r) => {
                self.ban_inj_call(s_l);
                self.ban_inj_call(s_r);
            }
            _ => {}
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

    fn check_dup(statement: &Statement) {
        match statement {
            Statement::Mut(x, _, e) | Statement::IndexedMut(x, _, _, e) if e.has_variable(x) => {
                panic!("variable `{}` appears on the both sides", x.0)
            }
            Statement::Call(_, xs) | Statement::Uncall(_, xs) => {
                let mut set = BTreeSet::new();
                for x in xs {
                    match set.get(x) {
                        None => {
                            set.insert(x);
                        }
                        Some(_) => panic!("variable `{}` is passed more than once", x.0),
                    }
                }
            }
            Statement::IfThenElseFi(_, s_l, s_r, _)
            | Statement::FromDoLoopUntil(_, s_l, s_r, _)
            | Statement::IfThenElse(_, s_l, s_r)
            | Statement::Sequence(s_l, s_r) => {
                Self::check_dup(s_l);
                Self::check_dup(s_r);
            }
            Statement::LocalDelocal(_, _, s, _, _) | Statement::For(For { statement: s, .. }) => {
                Self::check_dup(s)
            }
            _ => {}
        }
    }

    fn check_main(&self, main: &MainProc) {
        let MainProc(decls, statement, invl) = main;
        self.ban_inj_call(invl);

        Self::check_dup(statement);
        Self::check_dup(invl);

        let mut mutables = Self::mutables(decls.iter().map(|(t_x, _)| t_x));
        invl.check_mut(&mut mutables);
    }

    fn check_proc(&self, proc: &Proc) {
        match proc {
            Proc::Invl(_, params, statement, invl) => {
                self.ban_inj_call(invl);
                Self::check_dup(statement);
                Self::check_dup(invl);

                let mut mutables = Self::mutables(params);
                invl.check_mut(&mut mutables);
            }
            Proc::Inj(_, _, statement) => Self::check_dup(statement),
            Proc::Mat(_, _) => {}
        }
    }
}
