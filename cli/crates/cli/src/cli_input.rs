use clap::{arg, command, value_parser, Arg, ArgAction, ArgGroup, Command, Parser};
use indoc::indoc;

#[derive(Debug, Parser)]
pub(crate) struct DevCommand {
    /// Use a specific port
    #[arg(short, long, default_value_t = 4000)]
    pub(crate) port: u16,
    /// If a given port is unavailable, search for another
    #[arg(short, long)]
    pub(crate) search: bool,
    /// Do not listen for schema changes and reload
    #[arg(long)]
    pub(crate) disable_watch: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct CompletionsCommand {
    /// The shell to generate completions for.
    /// Supported: bash, fish, zsh, elvish, powershell
    pub(crate) shell: String,
}

#[derive(Debug, Parser)]
pub(crate) struct InitCommand {
    /// The name of the project to create
    pub(crate) name: String,
    /// The name or GitHub URL of the template to use for the new project
    #[arg(short, long)]
    pub(crate) template: String,
}

#[derive(Debug, Parser)]
pub(crate) struct CreateCommand {
    /// The name to use for the new project
    pub(crate) name: String,
    /// The slug of the account in which the new project should be created
    pub(crate) account: String,
    /// The regions in which the database for the new project should be created
    pub(crate) regions: Vec<String>,
}

#[derive(Debug, Parser)]
pub(crate) enum SubCommand {
    /// Run your Grafbase project locally
    Dev(DevCommand),
    /// Output completions for the chosen shell
    /// To use, write the output to the appropriate location for your shell
    Completions(CompletionsCommand),
    /// Sets up the current or a new project for Grafbase
    Init(InitCommand),
    /// Resets the local database for the current project
    Reset,
    /// Logs into your Grafbase account
    Login,
    /// Logs out of your Grafbase account
    Logout,
    /// Set up and deploy a new project
    Create(CreateCommand),
    /// Deploy your project
    Deploy,
    /// Connect a local project to a remote project
    Link,
    /// Disconnect a local project from a remote project
    Unlink,
}

#[derive(Debug, Parser)]
pub(crate) struct Args {
    /// Set tracing level
    #[arg(short, long, default_value_t = 0)]
    pub(crate) trace: u16,
    #[command(subcommand)]
    pub(crate) command: SubCommand,
}

/// creates the cli interface
#[must_use]
pub fn build_cli() -> Command {
    command!()
        .arg(
            arg!(-t --trace <level> "Set tracing level")
                .default_value("0")
                .value_parser(value_parser!(u16)),
        )
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("dev").about("Run your grafbase project locally").args(&[
                arg!(-p --port <port> "Use a specific port")
                    .default_value("4000")
                    .value_parser(value_parser!(u16)),
                arg!(-s --search "If a given port is unavailable, search for another").action(ArgAction::SetTrue),
                Arg::new("disable-watch")
                    .long("disable-watch")
                    .action(ArgAction::SetTrue)
                    .help("Do not listen for schema changes and reload"),
            ]),
        )
        .subcommand(
            Command::new("completions")
                .arg(Arg::new("shell").help(indoc! {"
                        The shell to generate completions for.
                        Supported: bash, fish, zsh, elvish, powershell
                    "}))
                .arg_required_else_help(true)
                .about(indoc! {"
                    Output completions for the chosen shell
                    To use, write the output to the appropriate location for your shell
                "}),
        )
        .subcommand(
            Command::new("init")
                .args(&[
                    arg!([name] "The name of the project to create"),
                    arg!(-t --template <name> "The name or GitHub URL of the template to use for the new project"),
                ])
                .about(indoc! {"
                    Sets up the current or a new project for Grafbase
                "}),
        )
        .subcommand(Command::new("reset").about(indoc! {"
            Resets the local database for the current project
        "}))
        .subcommand(Command::new("login").about("Logs into your Grafbase account"))
        .subcommand(Command::new("logout").about("Logs out of your Grafbase account"))
        .subcommand(
            Command::new("create").about("Set up and deploy a new project").args(&[
                arg!(-n --name <name> "The name to use for the new project"),
                arg!(-a --account <slug> "The slug of the account in which the new project should be created"),
                arg!(-r --regions <region> "The regions in which the database for the new project should be created")
            ])
            .group(ArgGroup::new("create")
            .args(["regions", "account", "name"])
            .multiple(true)
            .requires_all(["regions", "account", "name"])),
        )
        .subcommand(Command::new("deploy").about("Deploy your project"))
        .subcommand(Command::new("link").about("Connect a local project to a remote project"))
        .subcommand(Command::new("unlink").about("Disconnect a local project from a remote project"))
    // .subcommand(Command::new("logs").about("TBD"))
    // // TODO: schema edit / view
}

#[test]
fn verify_cli() {
    build_cli().debug_assert();
}
