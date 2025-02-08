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
        Self::new(program).check_program(program);
    }

    fn new(program: &Program) -> Self {
        let Program(_, procs) = program;
        let mut types = BTreeMap::<ProcId, ProcType>::new();
        for proc in procs {
            if let (id, Some(_)) = match proc {
                Proc::Inj(id, _, _) => (id, types.insert(id.clone(), ProcType::Inj)),
                Proc::Invl(id, _, _, _) | Proc::Mat(id, _) => {
                    (id, types.insert(id.clone(), ProcType::Invl))
                }
            } {
                panic!("colliding function names: {}", id.0);
            }
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
            None => panic!("undefined function found: {:?}", id.0)
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
