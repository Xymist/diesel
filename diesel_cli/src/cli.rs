use clap::{App, AppSettings, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    let database_arg = Arg::with_name("DATABASE_URL")
        .long("database-url")
        .help(
            "Specifies the database URL to connect to. Falls back to \
             the DATABASE_URL environment variable if unspecified.",
        )
        .global(true)
        .takes_value(true);

    let migration_subcommand = SubCommand::with_name("migration")
        .about(
            "A group of commands for generating, running, and reverting \
             migrations.",
        )
        .setting(AppSettings::VersionlessSubcommands)
        .arg(migration_dir_arg())
        .subcommand(SubCommand::with_name("run").about("Runs all pending migrations"))
        .subcommand(SubCommand::with_name("revert").about("Reverts the latest run migration"))
        .subcommand(SubCommand::with_name("redo").about(
            "Reverts and re-runs the latest migration. Useful \
             for testing that a migration can in fact be reverted.",
        ))
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists all available migrations, marking those that have been applied."),
        )
        .subcommand(
            SubCommand::with_name("pending")
                .about("Returns true if there are any pending migrations."),
        )
        .subcommand(
            SubCommand::with_name("generate")
                .about(
                    "Generate a new migration with the given name, and \
                     the current timestamp as the version",
                )
                .arg(
                    Arg::with_name("MIGRATION_NAME")
                        .help("The name of the migration to create")
                        .required(true),
                )
                .arg(
                    Arg::with_name("MIGRATION_VERSION")
                        .long("version")
                        .help(
                            "The version number to use when generating the migration. \
                             Defaults to the current timestamp, which should suffice \
                             for most use cases.",
                        )
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("MIGRATION_FORMAT")
                        .long("format")
                        .possible_values(&["sql", "barrel"])
                        .default_value("sql")
                        .takes_value(true)
                        .help("The format of the migration to be generated."),
                ),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp);

    let setup_subcommand = SubCommand::with_name("setup")
        .arg(migration_dir_arg())
        .about(
            "Creates the migrations directory, creates the database \
             specified in your DATABASE_URL, and runs existing migrations.",
        );

    let database_subcommand = SubCommand::with_name("database")
        .alias("db")
        .arg(migration_dir_arg())
        .about("A group of commands for setting up and resetting your database.")
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(SubCommand::with_name("setup").about(
            "Creates the database specified in your DATABASE_URL, \
             and then runs any existing migrations.",
        ))
        .subcommand(SubCommand::with_name("reset").about(
            "Resets your database by dropping the database specified \
             in your DATABASE_URL and then running `diesel database setup`.",
        ))
        .subcommand(
            SubCommand::with_name("drop")
                .about("Drops the database specified in your DATABASE_URL.")
                .setting(AppSettings::Hidden),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp);

    let generate_bash_completion_subcommand = SubCommand::with_name("bash-completion")
        .about("Generate bash completion script for the diesel command.");

    let infer_schema_subcommand = SubCommand::with_name("print-schema")
        .setting(AppSettings::VersionlessSubcommands)
        .about("Print table definitions for database schema.")
        .arg(
            Arg::with_name("schema")
                .long("schema")
                .short("s")
                .takes_value(true)
                .help("The name of the schema."),
        )
        .arg(
            Arg::with_name("table-name")
                .index(1)
                .takes_value(true)
                .multiple(true)
                .help("Table names to filter (default only-tables if not empty)"),
        )
        .arg(
            Arg::with_name("only-tables")
                .short("o")
                .long("only-tables")
                .help("Only include tables from table-name")
                .conflicts_with("except-tables")
                .conflicts_with("blacklist"),
        )
        .arg(
            Arg::with_name("whitelist")
                .short("w")
                .long("whitelist")
                .hidden(true)
                .conflicts_with("blacklist")
                .conflicts_with("except-tables"),
        )
        .arg(
            Arg::with_name("except-tables")
                .short("e")
                .long("except-tables")
                .help("Exclude tables from table-name")
                .conflicts_with("only-tables")
                .conflicts_with("whitelist"),
        )
        .arg(
            Arg::with_name("blacklist")
                .short("b")
                .long("blacklist")
                .hidden(true)
                .conflicts_with("whitelist")
                .conflicts_with("only-tables"),
        )
        .arg(
            Arg::with_name("with-docs")
                .long("with-docs")
                .help("Render documentation comments for tables and columns"),
        )
        .arg(
            Arg::with_name("patch-file")
                .long("patch-file")
                .takes_value(true)
                .help("A unified diff file to be applied to the final schema"),
        )
        .arg(
            Arg::with_name("import-types")
                .long("import-types")
                .takes_value(true)
                .multiple(true)
                .number_of_values(1)
                .help("A list of types to import for every table, separated by commas"),
        );

    let config_arg = Arg::with_name("CONFIG_FILE")
        .long("config-file")
        .help(
            "The location of the configuration file to use. Falls back to the \
             `DIESEL_CONFIG_FILE` environment variable if unspecified. Defaults \
             to `diesel.toml` in your project root. See \
             diesel.rs/guides/configuring-diesel-cli for documentation on this file.",
        )
        .global(true)
        .takes_value(true);

    App::new("diesel")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::VersionlessSubcommands)
        .after_help(
            "You can also run `diesel SUBCOMMAND -h` to get more information about that subcommand.",
        )
        .arg(database_arg)
        .arg(config_arg)
        .subcommand(migration_subcommand)
        .subcommand(setup_subcommand)
        .subcommand(database_subcommand)
        .subcommand(generate_bash_completion_subcommand)
        .subcommand(infer_schema_subcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
}

fn migration_dir_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("MIGRATION_DIRECTORY")
        .long("migration-dir")
        .help(
            "The location of your migration directory. By default this \
             will look for a directory called `migrations` in the \
             current directory and its parents.",
        )
        .takes_value(true)
        .global(true)
}
