use std::collections::HashMap;

const USER: &str = "user";
const GROUP: &str = "group";
const FILE: &str = "file";

pub fn build_registry() -> HashMap<&'static str, Box<dyn ProviderBuilder>> {
    HashMap::from([
        (
            USER,
            Box::new(UserBuilder::new()) as Box<dyn ProviderBuilder>,
        ),
        (
            GROUP,
            Box::new(GroupBuilder::new()) as Box<dyn ProviderBuilder>,
        ),
        (
            FILE,
            Box::new(FileBuilder::new()) as Box<dyn ProviderBuilder>,
        ),
    ])
}

pub struct Registry {
    entries: HashMap<String, Box<dyn ProviderBuilder>>,
}

impl Registry {
    fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    fn find(&self, s: &String) -> Option<&Box<dyn ProviderBuilder>> {
        let builder = self.entries.get(s);
        match builder {
            Some(b) => Some(b),
            None => panic!("invalid builder"),
        }
    }
}

fn main() {
    use ProviderResult::{NotImplemented as ResultNotImplemented, Success as ResultSuccess};

    let registry = build_registry();

    match registry.get(USER) {
        Some(builder) => {
            let _ = builder.build().run();
        }
        None => panic!("invalid provider"),
    }

    match registry.get(GROUP) {
        Some(builder) => {
            let provider = builder.build();

            match provider.init() {
                Ok(res) => match res {
                    ResultNotImplemented => println!("not implemented"),
                    ResultSuccess(_) => println!("success"),
                },
                Err(err) => match err {
                    ProviderError::NotImplemented => println!("not implemented"),
                },
            }

            match provider.run() {
                Ok(res) => match res {
                    ResultNotImplemented => println!("not implemented"),
                    ResultSuccess(_) => println!("success"),
                },
                Err(err) => match err {
                    ProviderError::NotImplemented => println!("not implemented"),
                },
            }

            match provider.exit() {
                Ok(res) => match res {
                    ResultNotImplemented => println!("not implemented"),
                    ResultSuccess(_) => println!("success"),
                },
                Err(err) => match err {
                    ProviderError::NotImplemented => println!("not implemented"),
                },
            }
        }
        None => panic!("invalid provider"),
    }

    match registry.get(FILE) {
        Some(builder) => {
            let _ = builder.build().run();
        }
        None => panic!("invalid provider"),
    }
}

pub enum ProviderResult<T> {
    NotImplemented,
    Success(T),
}

pub enum RunStatus {
    NoBalance,
    LowAmount,
    HighAmount,
}

pub struct Init {}

pub struct Run {}

pub enum ProviderError {
    NotImplemented,
}

pub trait Provider {
    fn init(&self) -> Result<ProviderResult<Init>, ProviderError> {
        Err(ProviderError::NotImplemented)
    }

    fn run(&self) -> Result<ProviderResult<Run>, ProviderError> {
        Err(ProviderError::NotImplemented)
    }

    fn exit(&self) -> Result<ProviderResult<Run>, ProviderError> {
        Err(ProviderError::NotImplemented)
    }
}

#[derive(Debug)]
pub struct UserProvider {}

impl Provider for UserProvider {
    fn run(&self) -> Result<ProviderResult<Run>, ProviderError> {
        println!("configuring user");
        Err(ProviderError::NotImplemented)
    }
}

#[derive(Debug)]
pub struct GroupProvider {}

impl Provider for GroupProvider {
    fn run(&self) -> Result<ProviderResult<Run>, ProviderError> {
        println!("configure group");
        Err(ProviderError::NotImplemented)
    }
}

#[derive(Debug)]
pub struct FileProvider {}

impl Provider for FileProvider {
    fn run(&self) -> Result<ProviderResult<Run>, ProviderError> {
        println!("configure file");
        Err(ProviderError::NotImplemented)
    }
}

pub trait ProviderBuilder {
    fn build(&self) -> Box<dyn Provider>;
}

pub struct GroupBuilder;

impl GroupBuilder {
    fn new() -> Self {
        Self
    }
}

impl ProviderBuilder for GroupBuilder {
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

impl ProviderBuilder for UserBuilder {
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

impl ProviderBuilder for FileBuilder {
    fn build(&self) -> Box<dyn Provider> {
        Box::new(FileProvider {})
    }
}
