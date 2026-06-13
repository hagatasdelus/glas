use super::cli::Cli;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputLayout {
    Long,
    Custom,
    OneLine,
    Grid,
}

pub fn resolve_layout_mode(cli: &Cli, stdout_is_tty: bool) -> OutputLayout {
    if cli.long {
        return OutputLayout::Long;
    }
    if cli.format.is_some() {
        return OutputLayout::Custom;
    }
    if cli.oneline || cli.null {
        return OutputLayout::OneLine;
    }
    if stdout_is_tty {
        OutputLayout::Grid
    } else {
        OutputLayout::OneLine
    }
}
