use std::collections::HashMap;

const USER: &str = "user";
const GROUP: &str = "group";
const FILE: &str = "file";

pub fn registry() -> HashMap<&'static str, Box<dyn Builder>> {
    HashMap::from([
        (USER, Box::new(UserBuilder::new()) as Box<dyn Builder>),
        (GROUP, Box::new(GroupBuilder::new()) as Box<dyn Builder>),
        (FILE, Box::new(FileBuilder::new()) as Box<dyn Builder>),
    ])
}

fn main() {
    let reg = registry();

    match reg.get(USER) {
        Some(builder) => {
            let provider = builder.build();
            let _ = provider.run();
        }
        None => panic!("invalid provider"),
    }

    match reg.get(GROUP) {
        Some(builder) => {
            let provider = builder.build();
            let _ = provider.run();
        }
        None => panic!("invalid provider"),
    }

    match reg.get(FILE) {
        Some(builder) => {
            let provider = builder.build();
            let _ = provider.run();
        }
        None => panic!("invalid provider"),
    }
}

pub enum RunStatus {
    Success,
    Failure(RunReason),
    Pending,
}

pub enum RunReason {
    InvalidCommand,
}

pub struct RunResult {}

pub enum RunError {
    NotImplemented,
}

pub trait Provider {
    fn run(&self) -> Result<RunResult, RunError> {
        Err(RunError::NotImplemented)
    }
}

#[derive(Debug)]
pub struct UserProvider {}

impl Provider for UserProvider {
    fn run(&self) -> Result<RunResult, RunError> {
        println!("configuring user");
        Ok(RunResult {})
    }
}

#[derive(Debug)]
pub struct GroupProvider {}

impl Provider for GroupProvider {
    fn run(&self) -> Result<RunResult, RunError> {
        println!("configure group");
        Ok(RunResult {})
    }
}

#[derive(Debug)]
pub struct FileProvider {}

impl Provider for FileProvider {
    fn run(&self) -> Result<RunResult, RunError> {
        println!("configure file");
        Ok(RunResult {})
    }
}

pub trait Builder {
    fn build(&self) -> Box<dyn Provider>;
}

pub struct GroupBuilder;

impl GroupBuilder {
    fn new() -> Self {
        Self
    }
}

impl Builder for GroupBuilder {
    fn build(&self) -> Box<dyn Provider> {
        Box::new(GroupProvider {})
    }
}

pub struct UserBuilder;

impl UserBuilder {
    fn new() -> Self {
        Self
    }
}

impl Builder for UserBuilder {
    fn build(&self) -> Box<dyn Provider> {
        Box::new(UserProvider {})
    }
}

pub struct FileBuilder;

impl FileBuilder {
    fn new() -> Self {
        Self
    }
}

impl Builder for FileBuilder {
    fn build(&self) -> Box<dyn Provider> {
        Box::new(FileProvider {})
    }
}
