use crate::parser::detail::{MainProc, Proc, ProcId, Program, Statement};
use std::collections::BTreeMap;

#[derive(Debug)]
enum ProcType {
    Inj,
    Invl,
}

#[derive(Debug)]
pub struct Checker {
    types: BTreeMap<ProcId, ProcType>,
}

impl Checker {
    pub fn check(program: &Program) {
        let checker = Self::new(program);
        checker.check_program(program);
    }

    fn new(program: &Program) -> Self {
        let Program(_, procs) = program;
        let mut types = BTreeMap::<ProcId, ProcType>::new();
        for proc in procs {
            match proc {
                Proc::Inj(id, _, _) => types.insert(id.clone(), ProcType::Inj),
                Proc::Invl(id, _, _, _) => types.insert(id.clone(), ProcType::Invl),
            };
        }
        Self { types }
    }

    fn check_program(&self, program: &Program) {
        let Program(main, procs) = program;
        self.check_main(main);
        for proc in procs {
            self.check_proc(proc);
        }
    }

    fn verify(&self, id: &ProcId) {
        match self.types.get(id) {
            Some(ProcType::Inj) => panic!("expected invl, found inj"),
            Some(ProcType::Invl) => {}
            None => unreachable!(),
        }
    }

    fn check_main(&self, main: &MainProc) {
        if let MainProc(_, _, Statement::Call(id, _)) = main {
            self.verify(id);
        }
    }

    fn check_proc(&self, proc: &Proc) {
        if let Proc::Invl(_, _, _, Statement::Call(id, _)) = proc {
            self.verify(id);
        }
    }
}
