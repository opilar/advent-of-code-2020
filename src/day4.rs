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
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
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
        "byr" => builder.with_byr(value),
        "iyr" => builder.with_iyr(value),
        "eyr" => builder.with_eyr(value),
        "hgt" => builder.with_hgt(value),
        "hcl" => builder.with_hcl(value),
        "ecl" => builder.with_ecl(value),
        "pid" => builder.with_pid(value),
        "cid" => builder.with_cid(value),
        _ => panic!("unexpected pair key: {:?}", key),
    }
}

#[derive(Default)]
struct PassportBuilder {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl PassportBuilder {
    fn with_byr(mut self, byr: String) -> Self {
        self.byr = Some(byr);
        self
    }

    fn with_iyr(mut self, iyr: String) -> Self {
        self.iyr = Some(iyr);
        self
    }

    fn with_eyr(mut self, eyr: String) -> Self {
        self.eyr = Some(eyr);
        self
    }

    fn with_hgt(mut self, hgt: String) -> Self {
        self.hgt = Some(hgt);
        self
    }

    fn with_hcl(mut self, hcl: String) -> Self {
        self.hcl = Some(hcl);
        self
    }

    fn with_ecl(mut self, ecl: String) -> Self {
        self.ecl = Some(ecl);
        self
    }

    fn with_pid(mut self, pid: String) -> Self {
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

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Batch {
    Batch::parse(input)
}

#[aoc(day4, part1)]
fn part1(batch: &Batch) -> usize {
    batch.passports.len()
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
                ecl: "gry".to_owned(),
                pid: "860033327".to_owned(),
                eyr: "2020".to_owned(),
                hcl: "#fffffd".to_owned(),
                byr: "1937".to_owned(),
                iyr: "2017".to_owned(),
                cid: Some("147".to_owned()),
                hgt: "183cm".to_owned(),
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
                ecl: "brn".to_owned(),
                pid: "760753108".to_owned(),
                eyr: "2024".to_owned(),
                hcl: "#ae17e1".to_owned(),
                byr: "1931".to_owned(),
                iyr: "2013".to_owned(),
                cid: None,
                hgt: "179cm".to_owned(),
            })
        );

        assert_eq!(
            Passport::parse("hcl:#cfa07d eyr:2025 pid:166559648\niyr:2011 ecl:brn hgt:59in"),
            None
        );
    }
}
