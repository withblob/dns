use serde::{Deserialize, Serialize};
use reqwest::{Client};
use crate::options::Options;
use crate::config::*;

// This enum holds all possible DNS record types, but some are not supported yet.
#[derive(Clone, Debug, Serialize)]
pub enum RecordType {
    A,
    AAAA,
    CAA,
    CNAME,
    HINFO,
    LOC,
    MX,
    NAPTR,
    NS,
    OPT,
    PTR,
    SOA,
    SRV,
    SSHFP,
    TLSA,
    TXT,
}

impl RecordType {
    fn values() -> Vec<RecordType> {
        vec!(RecordType::A,
             RecordType::AAAA,
             RecordType::CNAME,
             RecordType::HINFO,
             RecordType::LOC,
             RecordType::MX,
             RecordType::NAPTR,
             RecordType::NS,
             RecordType::OPT,
             RecordType::PTR,
             RecordType::SOA,
             RecordType::SRV,
             RecordType::SSHFP,
             RecordType::TLSA,
             RecordType::TXT)
    }
    fn supported_values()-> Vec<RecordType> {
        vec!(RecordType::A, RecordType::AAAA,
             RecordType::CAA, RecordType::CNAME,
             RecordType::MX, RecordType::NS,
             RecordType::PTR, RecordType::SOA,
             RecordType::TXT)

    }

    pub fn default_value() -> RecordType {
        RecordType::A
    }

    pub fn parse(input: &str) -> Option<RecordType> {
        match input {
            "A" => Some(RecordType::A),
            "AAAA" => Some(RecordType::AAAA),
            "CAA" => Some(RecordType::CAA),
            "CNAME" => Some(RecordType::CNAME),
            "NS" => Some(RecordType::NS),
            "MX" => Some(RecordType::MX),
            "PTR" => Some(RecordType::PTR),
            "SOA" => Some(RecordType::SOA),
            "TXT" => Some(RecordType::TXT),
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        let s = match self {
            RecordType::A => "A",
            RecordType::AAAA => "AAAA",
            RecordType::CAA => "CAA",
            RecordType::CNAME => "CNAME",
            RecordType::HINFO => "HINFO",
            RecordType::LOC => "LOC",
            RecordType::NAPTR => "NAPTR",
            RecordType::NS => "NS",
            RecordType::MX => "MX",
            RecordType::OPT => "OPT",
            RecordType::PTR => "PTR",
            RecordType::SOA => "SOA",
            RecordType::SRV => "SRV",
            RecordType::SSHFP => "SSHFP",
            RecordType::TLSA => "TLSA",
            RecordType::TXT => "TXT",
        };
        s.to_string()
    }
}

#[derive(Debug, Serialize)]
struct ResolveRequest {
    domains: Vec<String>,
    types: Vec<RecordType>,
}

impl ResolveRequest {
    fn from_options(opts: &Options) -> ResolveRequest {
        let mut rr = ResolveRequest {
            domains: opts.domains.clone(),
            types: vec![],
        };
        for t in &opts.types {
            match RecordType::parse(&t) {
                Some(rt) => rr.types.push(rt),
                None => ()
            }
        }
        rr
    }
}

#[derive(Debug, Deserialize)]
pub struct ResolveResult {
    pub status: String,
    #[serde(skip_deserializing)]
    pub country: String,
    pub items: Vec<ResolveResultItem>,
    pub error: Option<ResolveResultError>,
}


#[derive(Debug, Deserialize)]
pub struct ResolveResultError { }

#[derive(Debug, Deserialize)]
pub struct ResolveResultItem {
    pub domain: String,
    pub values: Vec<ResolveResultItemValue>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ResolveResultItemValue {
    A { data: String, ttl: u32 },
    AAAA { data: String, ttl: u32 },
    CAA { data: String, ttl: u32, flag: u32, tag: String },
    CNAME { data: String, ttl: u32 },
    MX { data: String, ttl: u32, preference: u16 },
    NS { data: String, ttl: u32 },
    PTR { data: String, ttl: u32 },
    SOA { ttl: u32, mname: String, rname: String, serial: u32, refresh: u32, retry: u32, expire: u32, minimum: u32 },
    TXT { data: String, ttl: u32 },
}

pub fn is_dns_record_type(candidate: &str) -> bool {
    for variant in RecordType::values() {
        if variant.to_string() == candidate {
            return true
        }
    }
    false
}

pub fn is_supported_dns_record_type(candidate: &str) -> bool {
    for variant in RecordType::supported_values() {
        if variant.to_string() == candidate {
            return true
        }
    }
    false
}

pub async fn resolve(gc: &GlobalConfig, options: &Options)
                     -> Result<Vec<ResolveResult>, &'static str> {
    let mut results: Vec<ResolveResult> = Vec::new();
    for rc in &gc.resolvers {
        match resolve_once(rc, options).await {
            Ok(res) => results.push(res),
            Err(_e) => {
                let res = ResolveResult {
                    status: "error".to_string(),
                    country: country_by_code(&rc.country_code),
                    items: vec![],
                    error: Some(ResolveResultError {}),
                };
                results.push(res);
            }
        }
    }
    Ok(results)
}

fn country_by_code(country_code: &str) -> String {
    return if country_code == "us" {
        "USA".to_string()
    } else if country_code == "de" {
        "Germany".to_string()
    } else if country_code == "sg" {
        "Singapore".to_string()
    } else {
        "Unknown".to_string()
    };
}

async fn resolve_once(rc: &ResolverConfig, options: &Options) -> Result<ResolveResult, reqwest::Error> {
    let payload = ResolveRequest::from_options(options);
    let http_client = Client::builder().build()?;
    let response = http_client.post(&rc.url).json(&payload).send().await?;
    let mut resolve_result: ResolveResult = response.json().await?;
    resolve_result.country = country_by_code(&rc.country_code);
    Ok(resolve_result)
}
