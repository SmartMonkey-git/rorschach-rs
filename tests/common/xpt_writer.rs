//! Minimal SAS XPORT v5 writer for edge cases that ReadStat-generated fixtures
//! can't express (several datasets, special missing values, short numerics).

pub enum Value {
    Num(f64),
    /// SAS missing value: b'.' for `.`, b'_' for `._`, b'A'..=b'Z' for `.A`..`.Z`
    Missing(u8),
    Str(&'static str),
}

pub struct Var {
    pub name: &'static str,
    pub numeric: bool,
    pub length: usize,
    pub format: &'static str,
}

impl Var {
    pub fn num(name: &'static str) -> Self {
        Var {
            name,
            numeric: true,
            length: 8,
            format: "",
        }
    }
    pub fn num_len(name: &'static str, length: usize) -> Self {
        Var {
            name,
            numeric: true,
            length,
            format: "",
        }
    }
    pub fn num_fmt(name: &'static str, format: &'static str) -> Self {
        Var {
            name,
            numeric: true,
            length: 8,
            format,
        }
    }
    pub fn chr(name: &'static str, length: usize) -> Self {
        Var {
            name,
            numeric: false,
            length,
            format: "",
        }
    }
}

pub struct Dataset {
    pub name: &'static str,
    pub vars: Vec<Var>,
    pub rows: Vec<Vec<Value>>,
}

fn padded(s: &str, len: usize) -> Vec<u8> {
    let mut v = s.as_bytes().to_vec();
    v.resize(len, b' ');
    v
}

fn header(kind: &str, digits: &str) -> Vec<u8> {
    let rec = format!("HEADER RECORD*******{kind:<8}HEADER RECORD!!!!!!!{digits:0<30}  ");
    assert_eq!(rec.len(), 80, "{rec}");
    rec.into_bytes()
}

fn pad_to_record(buf: &mut Vec<u8>) {
    while !buf.len().is_multiple_of(80) {
        buf.push(b' ');
    }
}

/// Encodes an f64 as an 8-byte IBM System/370 double.
pub fn f64_to_ibm(v: f64) -> [u8; 8] {
    if v == 0.0 {
        return [0; 8];
    }
    let sign = if v < 0.0 { 0x80u8 } else { 0 };
    let mut frac = v.abs();
    let mut exp = 0i32;
    while frac >= 1.0 {
        frac /= 16.0;
        exp += 1;
    }
    while frac < 1.0 / 16.0 {
        frac *= 16.0;
        exp -= 1;
    }
    let mut mantissa = (frac * 2f64.powi(56)).round() as u64;
    if mantissa >= 1 << 56 {
        mantissa >>= 4;
        exp += 1;
    }
    let mut out = [0u8; 8];
    out[0] = sign | (exp + 64) as u8;
    out[1..].copy_from_slice(&mantissa.to_be_bytes()[1..]);
    out
}

pub fn write(datasets: &[Dataset]) -> Vec<u8> {
    let mut buf = header("LIBRARY", "");
    let stamp = "01JAN24:00:00:00";
    buf.extend(padded(
        &format!(
            "{:<8}{:<8}{:<8}{:<8}{:<8}{:24}{stamp}",
            "SAS", "SAS", "SASLIB", "9.4", "Linux", ""
        ),
        80,
    ));
    buf.extend(padded(stamp, 80));

    for ds in datasets {
        buf.extend(header("MEMBER", "000000000000000001600000000140"));
        buf.extend(header("DSCRPTR", ""));
        buf.extend(padded(
            &format!(
                "{:<8}{:<8}{:<8}{:<8}{:<8}{:24}{stamp}",
                "SAS", ds.name, "SASDATA", "9.4", "Linux", ""
            ),
            80,
        ));
        buf.extend(padded(stamp, 80));
        buf.extend(header("NAMESTR", &format!("000000{:04}", ds.vars.len())));

        let mut position = 0usize;
        for (i, v) in ds.vars.iter().enumerate() {
            let mut ns = Vec::with_capacity(140);
            ns.extend((if v.numeric { 1i16 } else { 2 }).to_be_bytes());
            ns.extend(0i16.to_be_bytes());
            ns.extend((v.length as i16).to_be_bytes());
            ns.extend(((i + 1) as i16).to_be_bytes());
            ns.extend(padded(v.name, 8));
            ns.extend(padded("", 40)); // label
            ns.extend(padded(v.format, 8));
            ns.extend([0u8; 6]); // nfl, nfd, nfj
            ns.extend([0u8; 2]); // nfill
            ns.extend(padded("", 8)); // informat
            ns.extend([0u8; 4]); // nifl, nifd
            ns.extend((position as i32).to_be_bytes());
            ns.resize(140, 0);
            buf.extend(ns);
            position += v.length;
        }
        pad_to_record(&mut buf);

        buf.extend(header("OBS", ""));
        for row in &ds.rows {
            for (v, value) in ds.vars.iter().zip(row) {
                match value {
                    Value::Num(n) => buf.extend(&f64_to_ibm(*n)[..v.length]),
                    Value::Missing(code) => {
                        buf.push(*code);
                        buf.extend(std::iter::repeat_n(0, v.length - 1));
                    }
                    Value::Str(s) => buf.extend(padded(s, v.length)),
                }
            }
        }
        pad_to_record(&mut buf);
    }
    buf
}
