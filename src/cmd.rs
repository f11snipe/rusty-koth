use std::process::exit;
use std::thread;
use anyhow::Result;
use clap::ArgMatches;
use console::{style, Style};
use crate::koth;
use crate::data::CmdExit;
use tracing::{debug, info};
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};
use clap::{crate_version, ArgAction};
use clap::{Arg, Command};
// use tracing::info;

pub const BANNER: &str = r"
    B A N N E R
";
pub fn banner(v: &str, matches: &ArgMatches) {
    if !matches.get_flag("no_banner") {
        println!(
            "{}\n                    {}",
            style(BANNER).magenta(),
            style(v).dim()
        );
    }
}

pub fn tracing(matches: &ArgMatches) {
    let level = if matches.get_flag("verbose") {
        LevelFilter::INFO
    } else {
        LevelFilter::OFF
    };
    Registry::default()
        .with(tracing_tree::HierarchicalLayer::new(2))
        .with(
            EnvFilter::builder()
                .with_default_directive(level.into())
                .with_env_var("LOG")
                .from_env_lossy(),
        )
        .init();
}

const DEFAULT_ERR_EXIT_CODE: i32 = 1;
pub fn result_exit(res: Result<CmdExit>) {
    let exit_with = match res {
        Ok(cmd) => {
            if let Some(message) = cmd.message {
                let style = if exitcode::is_success(cmd.code) {
                    Style::new().green()
                } else {
                    Style::new().red()
                };
                eprintln!("{}", style.apply_to(message));
            }
            cmd.code
        }
        Err(e) => {
            debug!("{:?}", e);
            DEFAULT_ERR_EXIT_CODE
        }
    };
    exit(exit_with)
}

pub fn command() -> Command {
    Command::new("koth")
        .version(crate_version!())
        .about("KOTH")
        .arg(
            Arg::new("bind")
                .short('b')
                .long("bind")
                .value_name("HOST")
                .help("Host to bind to (0.0.0.0, 127.0.0.1, etc)")
                .default_value("127.0.0.1")
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Port to use")
                .default_value("9999")
        )
        .arg(
            Arg::new("data_file")
                .short('d')
                .long("data-file")
                .value_name("FILE")
                .help("Path to data file (json)")
                .default_value("./data.json")
        )
        .arg(
            Arg::new("king_file")
                .short('k')
                .long("king-file")
                .value_name("FILE")
                .help("File to monitor as king file")
                .default_value("./king.txt")
        )
        .arg(
            Arg::new("tick_points")
                .short('t')
                .long("tick-points")
                .value_name("VALUE")
                .help("Amount of points per tick")
                .default_value("1")
        )
        .arg(
            Arg::new("tick_interval")
                .short('i')
                .long("tick-interval")
                .value_name("INTERVAL")
                .help("Interval for main score loop (in ms)")
                .default_value("500")
        )
        .arg(
            Arg::new("no_banner")
                .short('B')
                .long("no-banner")
                .help("Don't show the banner")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show details about interactions")
                .action(ArgAction::SetTrue),
        )
}

pub fn run(matches: &ArgMatches) -> Result<CmdExit> {
    let m1 = matches.clone();
    let web = thread::spawn(move || {
        let host = m1.get_one::<String>("bind").unwrap();
        let port: i32 = m1.get_one::<String>("port").unwrap().parse().unwrap();
        let data_file = m1.get_one::<String>("data_file").unwrap();
        let king_file = m1.get_one::<String>("king_file").unwrap();
        let bind_addr = format!("{}:{}", host, port);
        println!("listening: {}", bind_addr);
        koth::listen_http(&bind_addr, &king_file, &data_file);
    });

    let m2 = matches.clone();
    let king = thread::spawn(move || {
        let data_file = m2.get_one::<String>("data_file").unwrap();
        let king_file = m2.get_one::<String>("king_file").unwrap();
        let tick_points: u32 = m2.get_one::<String>("tick_points").unwrap().parse().unwrap();
        let tick_interval: u64 = m2.get_one::<String>("tick_interval").unwrap().parse().unwrap();
        info!("Data file: {}", data_file);
        info!("King file: {}", king_file);
        info!("Scoring: {}/{}ms", tick_points, tick_interval);
        koth::watch_king(&king_file, &data_file, tick_points, tick_interval);
    });

    web.join().unwrap();
    king.join().unwrap();

    Ok(CmdExit {
        code: exitcode::OK,
        message: None,
    })
}

pub fn handle() {
    let app = command();

    let v = app.render_version();
    let matches = app.get_matches();

    // use info! or trace! etc. to log
    // to instrument use `#[tracing::instrument(level = "trace", skip(session), err)]`
    tracing(&matches);
    banner(&v, &matches);

    let res = run(&matches);

    result_exit(res);
}
