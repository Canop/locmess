use {
    super::*,
    crossterm::tty::IsTty,
    minimad::{OwningTemplateExpander, TextTemplate},
    termimad::*,
};

pub struct Printer {
    pub skin: MadSkin,
    pub terminal_width: usize,
}

impl Printer {
    pub fn new(args: &args::Args) -> Self {
        let terminal_width = terminal_size().0 as usize;
        let color = args.color.value().unwrap_or(!is_output_piped());
        let skin = skin::make_skin(color);
        Self {
            skin,
            terminal_width,
        }
    }
    pub fn print(
        &self,
        expander: OwningTemplateExpander,
        template: &str,
    ) {
        let template = TextTemplate::from(template);
        let text = expander.expand(&template);
        let fmt_text = FmtText::from_text(&self.skin, text, Some(self.terminal_width));
        print!("{}", fmt_text);
    }
}

fn is_output_piped() -> bool {
    !std::io::stdout().is_tty()
}
