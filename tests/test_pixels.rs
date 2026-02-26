use chrono::{DateTime, NaiveDateTime, Utc};
use phenopackets::schema::v2::Phenopacket;
use phenopackets::schema::v2::core::PhenotypicFeature;
use phenopackets::schema::v2::core::TimeElement;
use phenopackets::schema::v2::core::time_element::Element;
use prost_types::Timestamp;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::{Instant, SystemTime};

#[test]
fn test_pixel_questionnaire_result() {
    let pp_path = PathBuf::from_str("/Users/rouvenreuter/data/phq9/phenopackets").unwrap();
    let paths = fs::read_dir(pp_path).unwrap();

    let mut phenopackets = vec![];
    let mut u_p_types = HashSet::new();

    for path in paths {
        if let Ok(path) = path {
            let str_pp = fs::read_to_string(&path.path()).unwrap();
            let pp: Phenopacket = serde_json::from_str(&str_pp).unwrap();
            let p_types: HashSet<String> = pp
                .phenotypic_features
                .iter()
                .map(|pf| {
                    let a = pf.r#type.clone().unwrap();
                    a.id
                })
                .collect();
            u_p_types.extend(p_types);

            phenopackets.push(pp);
        }
    }

    for pp in phenopackets {
        for pf in pp.phenotypic_features {
            let oc = &pf.r#type.unwrap();
        }
    }
}

fn find_start_end(pfs: &[PhenotypicFeature]) -> (Timestamp, Timestamp) {
    let mut start = DateTime::<Utc>::from(SystemTime::now());
    let end = DateTime::<Utc>::from(SystemTime::now());

    for pf in pfs {
        match &pf.onset {
            None => {}
            Some(onset_te) => {
                if let Some(Element::Timestamp(ts)) = onset_te.element {
                    let ts_chrono = proto_timestamp_to_naive_datetime(&ts).unwrap();
                    if ts_chrono < start {
                        start = ts_chrono;
                    }
                }
            }
        }

        match &pf.resolution {
            None => {}
            Some(resolution_te) => {
                if let Some(Element::Timestamp(ts)) = resolution_te.element {
                    let ts_chrono = proto_timestamp_to_naive_datetime(&ts).unwrap();
                    if ts_chrono > end {
                        start = ts_chrono;
                    }
                }
            }
        }

        (start, end)
    }
}

fn proto_timestamp_to_naive_datetime(ts: &Timestamp) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp(ts.seconds, ts.nanos as u32)
}
