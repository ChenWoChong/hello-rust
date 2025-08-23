use std::error::Error;
use std::process::Command;

pub type BoxedError = Box<dyn Error + Send + Sync>;

pub trait Executor {
    fn run(&self) -> Result<Option<i32>, BoxedError>;
}

pub struct Shell<'a, 'b> {
    cmd: &'a str,
    args: &'b [&'a str],
}

impl<'a, 'b> Shell<'a, 'b> {
    pub fn new(cmd: &'a str, args: &'b [&'a str]) -> Self {
        Self { cmd, args }
    }
}

impl<'a, 'b> Executor for Shell<'a, 'b> {
    fn run(&self) -> Result<Option<i32>, BoxedError> {
        let res = Command::new(self.cmd).args(self.args).output()?;
        Ok(res.status.code())
    }
}

pub fn execute_generics(cmd: &impl Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

pub fn execute_trait_object(cmd: &dyn Executor) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

pub fn execute_boxed_trait_object(cmd: Box<dyn Executor>) -> Result<Option<i32>, BoxedError> {
    cmd.run()
}

#[cfg(test)]
mod tests {
    use crate::generic::shell::{
        Executor, Shell, execute_boxed_trait_object, execute_generics, execute_trait_object,
    };

    #[test]
    fn shell_should_work() {
        let shell = Shell::new("ls", &[]);
        let res = shell.run().unwrap();
        assert_eq!(res, Some(0));
    }

    #[test]
    fn shell_trait_object_should_work() {
        let shell = Shell::new("ls", &[]);

        let res = execute_generics(&shell).unwrap();
        assert_eq!(res, Some(0));

        let res = execute_trait_object(&shell).unwrap();
        assert_eq!(res, Some(0));

        let box_shell = Box::new(shell);
        let res = execute_boxed_trait_object(box_shell).unwrap();
        assert_eq!(res, Some(0));

        // shell has moved
        // let _res = shell.run();
    }
}
