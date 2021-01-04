use std::{u32, u64};

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Batch {
    Batch::parse(input)
}

#[aoc(day4, part1)]
fn part1(batch: &Batch) -> usize {
    batch.passports.len()
}

#[aoc(day4, part2)]
fn part2(batch: &Batch) -> usize {
    batch.passports.len()
}

struct Batch {
    passports: Vec<Passport>,
}

impl Batch {
    fn parse(input: &str) -> Batch {
        let passports = input
            .split("\n\n")
            .map(Passport::parse)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();
        Batch { passports }
    }
}

#[derive(Debug, PartialEq)]
struct Passport {
    byr: BirthYear,
    iyr: IssueYear,
    eyr: ExpirationYear,
    hgt: Height,
    hcl: HairColor,
    ecl: EyeColor,
    pid: PassportID,
    cid: Option<String>,
}

impl Passport {
    fn parse(s: &str) -> Option<Passport> {
        let mut builder = PassportBuilder::default();
        for line in s.lines() {
            for pair_str in line.split(' ') {
                let pair: Vec<&str> = pair_str.split(':').collect();
                if pair.len() != 2 {
                    panic!("unexpected pair: {:?}", pair_str);
                }
                builder = parse_pair(builder, pair[0], pair[1].to_owned());
            }
        }
        builder.build()
    }
}

fn parse_pair(builder: PassportBuilder, key: &str, value: String) -> PassportBuilder {
    match key {
        "byr" => {
            if let Some(year) = BirthYear::parse(&value) {
                builder.with_byr(year)
            } else {
                builder
            }
        }
        "iyr" => {
            if let Some(year) = IssueYear::parse(&value) {
                builder.with_iyr(year)
            } else {
                builder
            }
        }
        "eyr" => {
            if let Some(year) = ExpirationYear::parse(&value) {
                builder.with_eyr(year)
            } else {
                builder
            }
        }
        "hgt" => {
            if let Some(hgt) = Height::parse(&value) {
                builder.with_hgt(hgt)
            } else {
                builder
            }
        }
        "hcl" => {
            if let Some(hcl) = HairColor::parse(&value) {
                builder.with_hcl(hcl)
            } else {
                builder
            }
        }
        "ecl" => {
            if let Some(ecl) = EyeColor::parse(&value) {
                builder.with_ecl(ecl)
            } else {
                builder
            }
        }
        "pid" => {
            if let Some(pid) = PassportID::parse(&value) {
                builder.with_pid(pid)
            } else {
                builder
            }
        }
        "cid" => builder.with_cid(value),
        _ => panic!("unexpected pair key: {:?}", key),
    }
}

#[derive(Default)]
struct PassportBuilder {
    byr: Option<BirthYear>,
    iyr: Option<IssueYear>,
    eyr: Option<ExpirationYear>,
    hgt: Option<Height>,
    hcl: Option<HairColor>,
    ecl: Option<EyeColor>,
    pid: Option<PassportID>,
    cid: Option<String>,
}

impl PassportBuilder {
    fn with_byr(mut self, byr: BirthYear) -> Self {
        self.byr = Some(byr);
        self
    }

    fn with_iyr(mut self, iyr: IssueYear) -> Self {
        self.iyr = Some(iyr);
        self
    }

    fn with_eyr(mut self, eyr: ExpirationYear) -> Self {
        self.eyr = Some(eyr);
        self
    }

    fn with_hgt(mut self, hgt: Height) -> Self {
        self.hgt = Some(hgt);
        self
    }

    fn with_hcl(mut self, hcl: HairColor) -> Self {
        self.hcl = Some(hcl);
        self
    }

    fn with_ecl(mut self, ecl: EyeColor) -> Self {
        self.ecl = Some(ecl);
        self
    }

    fn with_pid(mut self, pid: PassportID) -> Self {
        self.pid = Some(pid);
        self
    }

    fn with_cid(mut self, cid: String) -> Self {
        self.cid = Some(cid);
        self
    }

    fn build(self) -> Option<Passport> {
        if self.byr.is_none()
            || self.iyr.is_none()
            || self.eyr.is_none()
            || self.hgt.is_none()
            || self.hcl.is_none()
            || self.ecl.is_none()
            || self.pid.is_none()
        {
            None
        } else {
            Some(Passport {
                byr: self.byr.unwrap(),
                iyr: self.iyr.unwrap(),
                eyr: self.eyr.unwrap(),
                hgt: self.hgt.unwrap(),
                hcl: self.hcl.unwrap(),
                ecl: self.ecl.unwrap(),
                pid: self.pid.unwrap(),
                cid: self.cid,
            })
        }
    }
}

#[derive(Debug, PartialEq)]
struct BirthYear {
    year: u64,
}

impl BirthYear {
    const BOUNDS: Bounds = Bounds {
        least: 1920,
        most: 2002,
    };

    fn parse(s: &str) -> Option<Self> {
        let res = parse_number_in_bounds(s, &BirthYear::BOUNDS);
        if let Some(year) = res {
            Some(BirthYear { year })
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct IssueYear {
    year: u64,
}

impl IssueYear {
    const BOUNDS: Bounds = Bounds {
        least: 2010,
        most: 2020,
    };

    fn parse(s: &str) -> Option<Self> {
        let res = parse_number_in_bounds(s, &IssueYear::BOUNDS);
        if let Some(year) = res {
            Some(IssueYear { year })
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
struct ExpirationYear {
    year: u64,
}

impl ExpirationYear {
    const BOUNDS: Bounds = Bounds {
        least: 2020,
        most: 2030,
    };

    fn parse(s: &str) -> Option<Self> {
        let res = parse_number_in_bounds(s, &ExpirationYear::BOUNDS);
        if let Some(year) = res {
            Some(ExpirationYear { year })
        } else {
            None
        }
    }
}

struct Bounds {
    least: u64,
    most: u64,
}

impl Bounds {
    fn in_bound(&self, num: u64) -> bool {
        num >= self.least && num <= self.most
    }
}

fn parse_number_in_bounds(s: &str, bounds: &Bounds) -> Option<u64> {
    if let Ok(num) = s.parse() {
        if bounds.in_bound(num) {
            Some(num)
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Debug, PartialEq)]
enum Height {
    HeightCm { centimeters: u64 },
    HeightIn { inches: u64 },
}

impl Height {
    fn parse(s: &str) -> Option<Self> {
        if s.ends_with(Height::CM_SUFFIX) {
            Height::parse_centimeters(s)
        } else if s.ends_with(Height::IN_SUFFIX) {
            Height::parse_inches(s)
        } else {
            None
        }
    }

    const CM_BOUNDS: Bounds = Bounds {
        least: 150,
        most: 193,
    };

    const IN_BOUNDS: Bounds = Bounds {
        least: 59,
        most: 76,
    };

    fn parse_centimeters(s: &str) -> Option<Self> {
        if let Some(cm_str) = s.strip_suffix(Height::CM_SUFFIX) {
            let res = parse_number_in_bounds(cm_str, &Height::CM_BOUNDS);
            if let Some(cm) = res {
                Some(Height::HeightCm { centimeters: cm })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_inches(s: &str) -> Option<Self> {
        if let Some(cm_str) = s.strip_suffix(Height::IN_SUFFIX) {
            let res = parse_number_in_bounds(cm_str, &Height::IN_BOUNDS);
            if let Some(inches) = res {
                Some(Height::HeightIn { inches })
            } else {
                None
            }
        } else {
            None
        }
    }

    const CM_SUFFIX: &'static str = "cm";
    const IN_SUFFIX: &'static str = "in";
}

#[derive(Debug, PartialEq)]
struct HairColor {
    color: u32,
}

impl HairColor {
    fn parse(s: &str) -> Option<Self> {
        if !s.starts_with('#') || s.len() != 7 {
            None
        } else {
            u32::from_str_radix(&s[1..], 16)
                .map(|c| HairColor { color: c })
                .ok()
        }
    }
}

#[derive(Debug, PartialEq)]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

impl EyeColor {
    fn parse(s: &str) -> Option<Self> {
        match s {
            "amb" => Some(EyeColor::Amb),
            "blu" => Some(EyeColor::Blu),
            "brn" => Some(EyeColor::Brn),
            "gry" => Some(EyeColor::Gry),
            "grn" => Some(EyeColor::Grn),
            "hzl" => Some(EyeColor::Hzl),
            "oth" => Some(EyeColor::Oth),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct PassportID {
    id: u32,
}

impl PassportID {
    fn parse(s: &str) -> Option<Self> {
        if s.len() != 9 {
            return None;
        }
        s.parse::<u32>().map(|id| PassportID { id }).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_passport() {
        assert_eq!(
            Passport::parse(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd\nbyr:1937 iyr:2017 cid:147 hgt:183cm"
            ),
            Some(Passport {
                ecl: EyeColor::Gry,
                pid: PassportID { id: 860033327 },
                eyr: ExpirationYear { year: 2020 },
                hcl: HairColor { color: 0xfffffd },
                byr: BirthYear { year: 1937 },
                iyr: IssueYear { year: 2017 },
                cid: Some("147".to_owned()),
                hgt: Height::HeightCm { centimeters: 183 },
            })
        );

        assert_eq!(
            Passport::parse(
                "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884\nhcl:#cfa07d byr:1929"
            ),
            None
        );

        assert_eq!(
            Passport::parse(
                "hcl:#ae17e1 iyr:2013\neyr:2024\necl:brn pid:760753108 byr:1931\nhgt:179cm"
            ),
            Some(Passport {
                ecl: EyeColor::Brn,
                pid: PassportID { id: 760753108 },
                eyr: ExpirationYear { year: 2024 },
                hcl: HairColor { color: 0xae17e1 },
                byr: BirthYear { year: 1931 },
                iyr: IssueYear { year: 2013 },
                cid: None,
                hgt: Height::HeightCm { centimeters: 179 },
            })
        );

        assert_eq!(
            Passport::parse("hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in"),
            None
        );
    }
}
