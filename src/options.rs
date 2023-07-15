use std::{env};
use crate::dns;
use crate::dns::RecordType;


pub struct Options {
    pub domains: Vec<String>,
    pub types: Vec<String>,
    pub version: bool,
    pub help: bool,
    pub none: bool,
}

impl Options {
    pub fn parse(args: env::Args) -> Result<Options, String> {
        let mut options = Options {
            none: true,
            domains: vec![],
            types: vec![],
            version: false,
            help: false
        };
        if args.len() < 2 {
            return Ok(options);
        }
        for arg in args.skip(1) {
            if arg == "-v" || arg == "--version" {
                options.version = true;
                return Ok(options)
            }
            if arg == "-h" || arg == "--help" {
                options.help = true;
                return Ok(options)
            }
            if dns::is_supported_dns_record_type(&arg) {
                options.types.push(arg);
                options.none = false
            } else if dns::is_dns_record_type(&arg) {
                let err_msg =
                    format!("{} record type is not yet supported. {}", arg,
                                "Email support@withblob.com to add it.");
                return Err(err_msg)
            } else {
                // treat anything else as a domain for now
                options.domains.push(arg);
            }
        }
        // if no types provided, using A
        if options.types.len() == 0 {
            options.types.push(RecordType::default_value().to_string());
            options.none = false
        }
        Ok(options)
    }
}
