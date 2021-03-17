use {
    argh::FromArgs,
    std::path::PathBuf,
};

#[derive(Debug, FromArgs)]
/// locmess answers the essential question you forgot to ask:
/// "What's the distribution of line lenghts in my source code ?".
///
/// Source at https://github.com/Canop/locmess
pub struct Args {

    #[argh(switch)]
    /// print the version
    pub version: bool,

    #[argh(option, default = "Default::default()")]
    /// color and style: 'yes', 'no' or 'auto' (auto should be good in most cases)
    pub color: BoolArg,

    #[argh(option, short = 't')]
    /// filter files by type, eg `-t js`
    pub file_type: Option<String>,

    #[argh(option, short = 'o')]
    /// list files with lines longer than this treshold
    pub over: Option<usize>,

    #[argh(positional)]
    /// the source code file or folder to analyze
    pub file: Option<PathBuf>,
}


/// An optional boolean for use in Argh
#[derive(Debug, Clone, Copy, Default)]
pub struct BoolArg(Option<bool>);

impl BoolArg {
    pub fn value(self) -> Option<bool> {
        self.0
    }
}

impl argh::FromArgValue for BoolArg {
    fn from_arg_value(value: &str) -> Result<Self, String> {
        match value.to_lowercase().as_ref() {
            "auto" => Ok(BoolArg(None)),
            "yes" => Ok(BoolArg(Some(true))),
            "no" => Ok(BoolArg(Some(false))),
            _ => Err(format!("Illegal value: {:?}", value)),
        }
    }
}
