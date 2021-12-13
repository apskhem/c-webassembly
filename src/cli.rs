use std::str::FromStr;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    /// Input file to be compiled
    file: String,
    /// Number of lines to read
    #[structopt(short = "o")]
    outfile: Option<String>,
    /// Optimization level
    #[structopt(short = "O")]
    opt_level: Option<OptLevel>,
}

impl Opt {
    pub fn file(&self) -> &str {
        return &self.file;
    }

    pub fn opt_level(&self) -> OptLevel {
        if let Some(level) = &self.opt_level {
            return level.clone();
        }
        else {
            return OptLevel::O3
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OptLevel {
    O0,
    O1,
    O2,
    O3,
    OS,
    OZ
}

impl FromStr for OptLevel {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "0" => Ok(OptLevel::O0),
            "1" => Ok(OptLevel::O1),
            "2" => Ok(OptLevel::O2),
            "3" => Ok(OptLevel::O3),
            "s" => Ok(OptLevel::OS),
            "z" => Ok(OptLevel::OZ),
            _ => Err(format!("cannot parse optimizatoin level of: {}", s).into()),
        };
    }
}