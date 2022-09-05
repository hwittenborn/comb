use clap::{Arg, Command};

mod message;
mod render;

fn main() {
    let cmd = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("render")
                .about("Render a template")
                .arg(Arg::new("file").help("The file to render").required(true)),
        )
        .get_matches();

    match cmd.subcommand() {
        Some(("render", args)) => render::render(args),
        _ => unreachable!(),
    }
}
