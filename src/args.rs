use std::path::Path;

#[derive(PartialEq, Debug, Clone)]
pub enum CommandLineError {
    NumberOfParameters,
    FileNotFound(String),
}

pub struct Png2RexArgs {
    pub input: String,
    pub output: String,
    pub flip_v: bool,
    pub flip_h: bool,
    pub resize: Option<(u32, u32)>,
}

fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}

pub fn parse_args(args: &[String]) -> Result<Png2RexArgs, CommandLineError> {
    // Transform arguments list by exclusing the first one (the executable path/name)
    // and adding a bool to each entry. If set to "true" that argument has been handled
    // and will be removed.
    let mut args_extended: Vec<(&String, bool)> = args.iter().skip(1).map(|a| (a, false)).collect();

    let mut flip_v = false;
    let mut flip_h = false;
    let mut resize = None;

    let len = args_extended.len();
    for i in 0..len {
        if args_extended[i].0 == "--flipv" {
            args_extended[i].1 = true;
            flip_v = true;
        } else if args_extended[i].0 == "--fliph" {
            args_extended[i].1 = true;
            flip_h = true;
        } else if args_extended[i].0 == "--resize" {
            if i + 2 >= len {
                return Err(CommandLineError::NumberOfParameters);
            }
            args_extended[i].1 = true;
            args_extended[i + 1].1 = true;
            args_extended[i + 2].1 = true;
            let w = args_extended[i + 1].0.parse::<u32>();
            let h = args_extended[i + 2].0.parse::<u32>();
            if w.is_err() || h.is_err() {
                return Err(CommandLineError::NumberOfParameters);
            }
            resize = Some((w.unwrap(), h.unwrap()));
        }
    }

    // Remove processed entries
    let args_cleaned: Vec<&String> = args_extended
        .iter()
        .filter(|(_, b)| !b)
        .map(|(s, _)| *s)
        .collect();

    if args_cleaned.len() != 2 {
        return Err(CommandLineError::NumberOfParameters);
    }
    if !file_exists(args_cleaned[0]) {
        return Err(CommandLineError::FileNotFound(args_cleaned[0].clone()));
    }

    Ok(Png2RexArgs {
        input: args_cleaned[0].clone(),
        output: args_cleaned[1].clone(),
        flip_v,
        flip_h,
        resize,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn no_args() {
        let r = parse_args(&[]);
        assert!(r.is_err());
        if let Err(e) = r {
            assert_eq!(e, CommandLineError::NumberOfParameters);
        }
    }

    #[test]
    fn existence() {
        assert_eq!(file_exists("nonsense_filename.blahblahblah"), false);
    }

    #[test]
    fn file_not_found() {
        let r = parse_args(&[String::new(), "flibble".to_string(), "flibble".to_string()]);
        assert!(r.is_err());
        if let Err(e) = r {
            assert_eq!(e, CommandLineError::FileNotFound("flibble".to_string()));
        }
    }

    #[test]
    fn valid() {
        assert!(parse_args(&[
            String::new(),
            "resources/kitty.png".to_string(),
            "kitty.rex".to_string()
        ])
        .is_ok());
    }

    #[test]
    fn really_valid() {
        if let Ok(p) = parse_args(&[
            String::new(),
            "resources/kitty.png".to_string(),
            "kitty.xp".to_string(),
        ]) {
            assert_eq!(p.input, "resources/kitty.png");
            assert_eq!(p.output, "kitty.xp");
        }
    }
}
