use std::io::Result;
use std::path::PathBuf;

mod indenter;

pub struct PhaseResult {
    name: String,
    outlog: PathBuf,
    errlog: PathBuf,
}

impl PhaseResult {
    pub fn new(name: &str, outlog: PathBuf, errlog: PathBuf) -> PhaseResult {
        let name = String::from(name);
        PhaseResult {
            name,
            outlog,
            errlog,
        }
    }

    pub fn display(self) -> Result<()> {
        use self::indenter::Indenter;
        use std::fs::File;
        use std::io::{copy, stdout};

        println!("---- {} {} ----", crate::CMDNAME, self.name);
        for logpath in &[self.outlog, self.errlog] {
            println!("+ {}:", logpath.display());
            copy(&mut File::open(logpath)?, &mut Indenter::from(stdout()))?;
        }

        Ok(())
    }
}
