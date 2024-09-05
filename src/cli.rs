use clap::Parser;

#[derive(Parser, Debug)]
pub struct Arguments {
    string: String,
}

impl Arguments {
    pub fn string (&self) -> &str {
        &self.string
    }
}

pub struct CLI;

impl CLI {
    pub fn arguments () -> Arguments {
        Arguments::parse()
    }
}