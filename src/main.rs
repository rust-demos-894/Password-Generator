use std::env;
use std::str::from_utf8;
use std::vec;
use rand::seq::SliceRandom;

#[derive(Debug, PartialEq)]
enum ComplexLevel {
    Simple,
    Medium,
    Complicated,
}

fn parse<T>(args: T) -> (ComplexLevel, u8)
where
    T: Iterator,
    T::Item: AsRef<str>, 
{
    let mut level = ComplexLevel::Medium;
    let mut len: u8 = 8;
    for arg in args {
        match arg.as_ref().to_string() {
            s if s.starts_with("len=") => {
                let mut splits = s.split("=").map(|t| t.to_string());
                len = splits.nth(1).unwrap().parse().unwrap();
            },

            lv if [1, 2, 3].contains(&lv.parse().unwrap_or(-1)) => {
                level = match lv.trim() {//why complains that level needs to be mutebale here?
                    "1" => ComplexLevel::Simple,
                    "3" => ComplexLevel::Complicated,
                    _ => ComplexLevel::Medium,
                }
            },
            _ => (),
        }
    }

    (level, len)
}

fn main() {
    let (lv, len) = parse(env::args());
    let cfg = Config::new(lv, len);
    let pwd = generator(cfg);

    println!("{pwd}");
}

struct Config {
    level: ComplexLevel,
    length: u8,
}

impl Config {
    fn new(level: ComplexLevel, length: u8) -> Self {
        Config {
            level,
            length,
        }
    }

    fn get_level(&self) -> &ComplexLevel {
        &self.level
    }

    fn get_len(&self) -> u8 {
        self.length
    }
}

fn generator(cfg: Config) -> String {
    let mut ret: Vec<u8> = Vec::new();

    let nums: Vec<u8> = (48..58).collect();
    let mut upper_class: Vec<u8> = (65..91).collect();
    let mut alphabets: Vec<u8> = (97..123).collect();
    let mut special: Vec<u8> = vec![33, 35, 36, 37, 38, 41, 64];

    let mut range = nums;
    if cfg.get_level() == &ComplexLevel::Medium {
        range.append(&mut alphabets);
    }
    if cfg.get_level() == &ComplexLevel::Complicated {
        range.append(&mut alphabets);
        range.append(&mut special);
    }

    let mut rng = rand::thread_rng();
    let mut i = 0;
    while i < cfg.get_len() {
        ret.push(*range.choose(&mut rng).unwrap());
        i+=1;
    }

    from_utf8(&ret).map(|r| r.to_string()).unwrap_or("Fatal Error".to_string())
}