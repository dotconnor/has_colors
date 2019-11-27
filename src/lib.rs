use regex::Regex;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::iter::FromIterator;

const COLORS_2: usize = 1;
const COLORS_16: usize = 4;
const COLORS_256: usize = 8;
const COLORS_16M: usize = 24;

const CI_NAMES: [&'static str; 4] = ["TRAVIS", "CIRCLECI", "APPVEYOR", "GITLAB_CI"];

pub fn internal_get_color_depth(env: HashMap<String, Option<String>, RandomState>) -> usize {
  if let Some(e) = env.get("FORCE_COLOR") {
    if let Some(r) = e {
      match r.as_ref() {
        "" | "1" | "true" => {
          return COLORS_16;
        }
        "2" => {
          return COLORS_256;
        }
        "3" => {
          return COLORS_16M;
        }
        _ => {
          return COLORS_2;
        }
      }
    }
  }

  if env.get("NO_COLOR").is_some() {
    return COLORS_2;
  }

  if let Some(e) = env.get("TERM") {
    if e.as_ref().unwrap_or(&"".to_owned()) == "DUMB" {
      return COLORS_2;
    }
  }

  if let Some(_) = env.get("TMUX") {
    return COLORS_256;
  }

  if let Some(_) = env.get("CI") {
    for ci_name in CI_NAMES.iter() {
      if let Some(_) = env.get(ci_name.to_owned()) {
        return COLORS_256;
      }
    }
    if let Some(ci) = env.get("CI_NAME") {
      if ci.as_ref().unwrap_or(&"".to_owned()) == "codeship" {
        return COLORS_256;
      }
    }
    return COLORS_2;
  }

  if let Some(teamcity_version) = env.get("TEAMCITY_VERSION") {
    let re = Regex::new(r"^(9\.(0*[1-9]\d*)\.|\d{2,}\.)").unwrap();
    if re.is_match(teamcity_version.as_ref().unwrap_or(&"".to_owned())) {
      return COLORS_16;
    } else {
      return COLORS_2;
    }
  }

  if let Some(term_program) = env.get("TERM_PROGRAM") {
    if let Some(program) = term_program {
      match program.as_ref() {
        "iTerm.app" => {
          match env.get("TERM_PROGRAM_VERSION") {
            None => {
              return COLORS_256;
            }
            Some(e) => {
              let re = Regex::new(r"^[0-2]\.").unwrap();
              if re.is_match(e.as_ref().unwrap_or(&"".to_owned())) {
                return COLORS_256;
              }
            }
          }
          return COLORS_16M;
        }
        "HyperTerm" | "MacTerm" => return COLORS_16M,
        "Apple_Terminal" => return COLORS_256,
        _ => {}
      }
    }
  }

  if let Some(term_check) = env.get("TERM") {
    if let Some(term) = term_check {
      let xterm_test = Regex::new(r"^xterm-256").unwrap();
      if xterm_test.is_match(term) {
        return COLORS_256;
      }

      let term_env = term.to_lowercase();

      match term_env.as_ref() {
        "eterm" | "cons25" | "console" | "cygwin" | "dtterm" | "gnome" | "hurd" | "jfbterm"
        | "konsole" | "kterm" | "mlterm" | "putty" | "st" => {
          return COLORS_16;
        }
        "mosh" | "rxvt-unicode-24bit" | "terminator" => {
          return COLORS_16M;
        }
        _ => {}
      }

      let terms_with_16_bit_support: [Regex; 8] = [
        Regex::new(r"ansi").unwrap(),
        Regex::new(r"ansi").unwrap(),
        Regex::new(r"ansi").unwrap(),
        Regex::new(r"ansi").unwrap(),
        Regex::new(r"ansi").unwrap(),
        Regex::new(r"ansi").unwrap(),
        Regex::new(r"ansi").unwrap(),
        Regex::new(r"ansi").unwrap(),
      ];

      for term_regex in terms_with_16_bit_support.iter() {
        if term_regex.is_match(term_env.as_ref()) {
          return COLORS_16;
        }
      }
    }
  }

  if let Some(colorterm_raw) = env.get("COLORTERM") {
    if let Some(colorterm) = colorterm_raw {
      if colorterm == &"truecolor" || colorterm == &"24bit" {
        return COLORS_16M;
      }
      return COLORS_16;
    }
  }

  COLORS_2
}

pub fn get_color_depth() -> usize {
  internal_get_color_depth(HashMap::from_iter(std::env::vars_os().map(|x| {
    let key = x
      .0
      .clone()
      .into_string()
      .expect("Enviroment contains from unexpected characters.");
    let value = x.1.clone().into_string().ok();
    return (key, value);
  })))
}

pub fn has_colors(depth: usize) -> bool {
  depth <= 2usize.pow(get_color_depth() as u32)
}
