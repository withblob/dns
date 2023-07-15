use std::{env, error::Error};
use ansi_term::Color::*;
use crate::config::GlobalConfig;
use crate::dns;
use crate::dns::*;
use crate::options::{Options};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn run() -> Result<(), Box<dyn Error>> {
    let gc = GlobalConfig::fetch().await?;
    let options = Options::parse(env::args())?;
    if options.version { return Ok(print_version()); }
    if options.none { return Ok(print_usage()); }
    let results = dns::resolve(&gc, &options).await?;
    print_results(&results);
    Ok(())
}

fn print_version() {
    println!("{} version {}",
             Green.bold().paint("Blob DNS"),
             Yellow.bold().paint(VERSION));
}

fn print_usage() {
    let usage = include_str!("resources/usage.txt")
        .replace("\\{b}", "\u{001b}[1m")
        .replace("\\{u}", "\u{001b}[4m")
        .replace("\\{c}", "\u{001b}[0m")
        .replace("\\{gr}", "\u{001b}[32m")
        .replace("\\{gr-b}", "\u{001b}[1;32m")
        .replace("\\{bl}", "\u{001b}[34m")
        .replace("\\{bl-b}", "\u{001b}[1;34m")
        .replace("\\{ye}", "\u{001b}[33m")
        .replace("\\{ye-b}", "\u{001b}[1;33m");
    println!("{}", usage);
}

fn record_style(record_type: RecordType) -> ansi_term::Style {
    return match record_type {
        RecordType::A => Green.bold(),
        RecordType::AAAA => Green.bold(),
        RecordType::CAA => Red.bold(),
        RecordType::CNAME => Yellow.bold(),
        RecordType::MX => Cyan.bold(),
        RecordType::NS => Red.bold(),
        RecordType::PTR => Red.bold(),
        RecordType::SOA => Purple.bold(),
        RecordType::TXT => Yellow.bold(),
        _ => Purple.bold(),
    };
}

fn print_results(results: &Vec<ResolveResult>) {
    for res in results.iter() {
        if res.status == "error" {
            // This convoluted align + fill is because I'm using ansi_term with filling
            println!("{} (blobdns server error)",
                     Red.bold().paint(format!("{:<10}", format!("{}:", &res.country))));
            continue;
        }
        if res.status == "ok" {
            if res.items.iter().all(|i| i.values.is_empty()) {
                println!("{}: nothing found\n", Yellow.bold().underline().paint(res.country.clone()));
                continue;
            }
            println!("\n{}:\n", Yellow.bold().underline().paint(res.country.clone()));
            for i in res.items.iter() {
                for v in i.values.iter() {
                    match v {
                        ResolveResultItemValue::A { data, ttl } => {
                            print_data_line(RecordType::A, &i.domain, *ttl, data);
                        }
                        ResolveResultItemValue::AAAA { data, ttl } => {
                            print_data_line(RecordType::AAAA, &i.domain, *ttl, data);
                        }
                        ResolveResultItemValue::CAA { data, ttl, flag, tag } => {
                            print_caa_line(&i.domain, *ttl, data, *flag, tag);
                        }
                        ResolveResultItemValue::CNAME { data, ttl } => {
                            print_data_line(RecordType::CNAME, &i.domain, *ttl, data);
                        }
                        ResolveResultItemValue::MX { data, preference, ttl } => {
                            print_mx_line(&i.domain, *ttl, data, *preference);
                        }
                        ResolveResultItemValue::NS { data, ttl } => {
                            print_data_line(RecordType::NS, &i.domain, *ttl, data);
                        }
                        ResolveResultItemValue::PTR { data, ttl } => {
                            print_data_line(RecordType::PTR, &i.domain, *ttl, data);
                        }
                        ResolveResultItemValue::SOA {
                            ttl, rname,
                            mname, serial, refresh, retry, expire, minimum
                        } => {
                            print_soa_line(&i.domain, *ttl, mname, rname, *serial,
                                           *refresh, *retry, *expire, *minimum);
                        }
                        ResolveResultItemValue::TXT { data, ttl } => {
                            print_data_line(RecordType::TXT, &i.domain, *ttl, data);
                        }
                    }
                }
            }
        }
    }
}

fn print_data_line(rt: RecordType, domain: &str, ttl: u32, data: &str) {
    println!("{} {} {} {}",
             record_style(rt.clone()).paint(rt.to_string()),
             Blue.bold().paint(domain),
             format_duration_hms(ttl),
             data);
}

fn print_caa_line(domain: &str, ttl: u32, data: &str, flag: u32, tag: &str) {
    let rt = RecordType::CAA;
    println!(r#"{} {} {} {} {} "{}""#,
             record_style(rt.clone()).paint(rt.to_string()),
             Blue.bold().paint(domain),
             format_duration_hms(ttl),
             tag,
             flag,
             data);
}

fn print_mx_line(domain: &str, ttl: u32, data: &str, preference: u16) {
    let rt = RecordType::MX;
    println!(r#"{} {} {} {} "{}""#,
             record_style(rt.clone()).paint(rt.to_string()),
             Blue.bold().paint(domain),
             format_duration_hms(ttl),
             preference,
             data);
}

fn print_soa_line(domain: &str, ttl: u32, mname: &str, rname: &str, serial: u32,
                  refresh: u32, retry: u32, expire: u32, minimum: u32) {
    let rt = RecordType::SOA;
    println!("{} {} {} {} {} {} {} {} {} {}",
             record_style(rt.clone()).paint(rt.to_string()),
             Blue.bold().paint(domain),
             format_duration_hms(ttl),
             mname, rname, serial,
             format_duration_hms(refresh),
             format_duration_hms(retry),
             format_duration_hms(expire),
             format_duration_hms(minimum));
}

fn format_duration_hms(seconds: u32) -> String {
    if seconds < 60 {
        format!("{}s", seconds)
    } else if seconds < 60 * 60 {
        format!("{}m{:02}s",
                seconds / 60,
                seconds % 60)
    } else if seconds < 60 * 60 * 24 {
        format!("{}h{:02}m{:02}s",
                seconds / 3600,
                (seconds % 3600) / 60,
                seconds % 60)
    } else {
        format!("{}d{}h{:02}m{:02}s",
                seconds / 86400,
                (seconds % 86400) / 3600,
                (seconds % 3600) / 60,
                seconds % 60)
    }
}


