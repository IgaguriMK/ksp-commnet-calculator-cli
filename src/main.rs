use anyhow::{Error, Result};
use clap::{crate_name, App, Arg, ArgMatches};

use ksp_commnet_calculator_core::antenna::Antennas;
use ksp_commnet_calculator_core::distance::Distances;
use ksp_commnet_calculator_core::endpoint::Endpoint;
use ksp_commnet_calculator_core::util::MetricPrefix;

const INDENT: &str = "    ";

const DEFAULT_FROM: &str = "DSN Lv.3";
const DEFAULT_TO: &str = "Command Module";

fn main() {
    if let Err(e) = w_main() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn w_main() -> Result<()> {
    let matches = App::new(crate_name!())
        .arg(
            Arg::with_name("from")
                .short("f")
                .long("from")
                .multiple(true)
                .takes_value(true)
                .default_value("DSN Lv.3"),
        )
        .arg(
            Arg::with_name("to")
                .short("t")
                .long("to")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("antennas")
                .short("A")
                .long("antennas")
                .help("Print antennas"),
        )
        .get_matches();

    let antennas = Antennas::new();

    if matches.is_present("antennas") {
        print_antennas(&antennas);
        return Ok(());
    }

    print_dists(matches, antennas)
}

fn print_antennas(antennas: &Antennas) {
    println!("Available antennas:");
    for a in antennas.iter() {
        print!("    {}", a.name);
        if !a.aliases.is_empty() {
            print!(" (");
            for (i, al) in a.aliases.iter().enumerate() {
                if i > 0 {
                    print!(", ");
                }
                print!("{}", al);
            }
            print!(")");
        }
        println!();
    }
}

fn print_dists(matches: ArgMatches, antennas: Antennas) -> Result<()> {
    let mut from = Endpoint::new();
    for antenna_str in matches.values_of("from").unwrap_or_default() {
        let (count, antenna_name) = split_antenna_arg(antenna_str)?;
        if let Some(antenna) = antennas.get(antenna_name) {
            from.add_antenna(antenna.clone(), count)
        }
    }
    if from.is_empty() {
        let a = antennas
            .get(DEFAULT_FROM)
            .expect("Default 'from' endpoint antenna not exists");
        from.add_antenna(a.clone(), 1);
    }

    let mut to = Endpoint::new();
    for antenna_str in matches.values_of("to").unwrap_or_default() {
        let (count, antenna_name) = split_antenna_arg(antenna_str)?;
        if let Some(antenna) = antennas.get(antenna_name) {
            to.add_antenna(antenna.clone(), count)
        }
    }
    if to.is_empty() {
        let a = antennas
            .get(DEFAULT_TO)
            .expect("Default 'to' endpoint antenna not exists");
        to.add_antenna(a.clone(), 1);
    }

    let range = from.range_to(&to);

    println!();
    println!(" From:");
    print_endpoint(&from);
    println!(" To:");
    print_endpoint(&to);
    println!();

    println!(" Max distance: {}m", MetricPrefix(range.max_distance()));
    println!();

    let dists = Distances::new();
    let strengths = dists.get_strengthes(range);

    println!(" |          Section          |   @Min   |   @Max   |");
    println!(" |:--------------------------|---------:|---------:|");
    for strength in &strengths {
        println!(
            " | {:<25} | {:>8} | {:>8} |",
            strength.section,
            format_strength(strength.at_min),
            format_strength(strength.at_max),
        );
    }
    println!();

    Ok(())
}

fn split_antenna_arg(s: &str) -> Result<(usize, &str)> {
    let parts: Vec<&str> = s.split(':').collect();

    match parts.len() {
        1 => Ok((1, parts[0])),
        2 => {
            let n = parts[1].parse()?;
            Ok((n, parts[0]))
        }
        _ => Err(Error::msg(format!(
            "antenna specifier should be [<NUMBER_OF_ANTENNA>:]<ANTENNA_NAME>, but {}",
            s
        ))),
    }
}

fn print_endpoint(endpoint: &Endpoint) {
    println!(" {}:", endpoint.endpoint_type());

    println!(" {}Power: {}", INDENT, MetricPrefix(endpoint.power()));

    println!(" {}Antennae:", INDENT);
    for (a, c) in endpoint.antenna_counts() {
        if c == 1 {
            println!(" {}{}{}", INDENT, INDENT, a.name);
        } else {
            println!(" {}{}{}x {}", INDENT, INDENT, c, a.name);
        }
    }
}

fn format_strength(strength: Option<f64>) -> String {
    if let Some(s) = strength {
        format!("{:.1} %", 100.0 * s)
    } else {
        "NA".to_owned()
    }
}
