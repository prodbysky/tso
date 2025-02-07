use clap::Parser;
use serde::Serialize;

#[derive(Debug, Default, Clone, clap::ValueEnum, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Transpile to python
    Python,
    /// Run the interpreter
    Interpret,
    /// Run the REPL
    #[default]
    Repl,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Repl => write!(f, "repl"),
            Self::Python => write!(f, "python"),
            Self::Interpret => write!(f, "interpret"),
        }
    }
}

/// The single executable tso language compiler/interpreter/transpiler
#[derive(Parser, Debug)]
pub struct Config {
    /// The mode of the program
    #[arg(default_value_t=Mode::Repl)]
    mode: Mode,

    // The input file to run/transpile
    #[arg(
        short,
        required_if_eq("mode", "python"),
        required_if_eq("mode", "interpret")
    )]
    input_file: Option<String>,

    // The output file to write to when python mode is selected
    #[arg(short, required_if_eq("mode", "python"))]
    output_file: Option<String>,
}

impl Config {
    pub fn mode(&self) -> &Mode {
        &self.mode
    }
    pub fn input(&self) -> Option<&str> {
        self.input_file.as_deref()
    }
    pub fn output(&self) -> Option<&str> {
        self.output_file.as_deref()
    }
}
