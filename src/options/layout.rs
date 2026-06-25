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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse_cli(args: &[&str]) -> Cli {
        let argv: Vec<&str> = std::iter::once("glas")
            .chain(args.iter().copied())
            .collect();
        Cli::parse_from(argv)
    }

    #[test]
    fn default_layout_is_grid_on_tty() {
        let cli = parse_cli(&[]);
        assert_eq!(resolve_layout_mode(&cli, true), OutputLayout::Grid);
    }

    #[test]
    fn default_layout_falls_back_to_oneline_when_not_tty() {
        let cli = parse_cli(&[]);
        assert_eq!(resolve_layout_mode(&cli, false), OutputLayout::OneLine);
    }
}
