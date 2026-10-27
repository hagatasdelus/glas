//! # gencomp
//!
//! Generates shell completion scripts for the `glas` CLI into an output
//! directory.

use clap::{Command, CommandFactory};
use clap_complete::Shell;
use std::path::Path;

/// Writes the completion script for one shell to `outdir/<shell subdir>/<file>`.
///
/// Panics when the destination directory cannot be created. Writing the
/// destination file is best-effort: when the file cannot be opened, the
/// shell simply gets no completion script.
fn generate_impl(s: Shell, app: &mut Command, appname: &str, outdir: &Path, file: String) {
    let destfile = outdir.join(file);
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    if let Ok(mut dest) = std::fs::File::create(destfile) {
        clap_complete::generate(s, app, appname, &mut dest);
    }
}

/// Generates completion scripts for all supported shells under `outdir`.
///
/// Layout matches the shipped release archives: `bash/`, `elvish/`, `fish/`,
/// `powershell/`, and `zsh/` (zsh uses the `_<appname>` filename convention),
/// invoked either by `--completions` or the packaging workflow.
pub fn generate(outdir: &Path) {
    use clap_complete::Shell::{Bash, Elvish, Fish, PowerShell, Zsh};
    let appname = "glas";
    let mut app = crate::options::Cli::command();
    app.set_bin_name(appname);
    generate_impl(Bash, &mut app, appname, outdir, format!("bash/{appname}"));
    generate_impl(
        Elvish,
        &mut app,
        appname,
        outdir,
        format!("elvish/{appname}"),
    );
    generate_impl(Fish, &mut app, appname, outdir, format!("fish/{appname}"));
    generate_impl(
        PowerShell,
        &mut app,
        appname,
        outdir,
        format!("powershell/{appname}"),
    );
    generate_impl(Zsh, &mut app, appname, outdir, format!("zsh/_{appname}"));
}
