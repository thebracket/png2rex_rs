use std::path::Path;

#[derive(PartialEq, Debug, Clone)]
pub enum CommandLineError {
    NumberOfParameters,
    FileNotFound(String),
}

pub struct Png2RexArgs {
    pub input: String,
    pub output: String,
}

fn file_exists(filename: &str) -> bool {
    Path::new(filename).exists()
}

pub fn parse_args(args: &[String]) -> Result<Png2RexArgs, CommandLineError> {
    if args.len() != 3 {
        return Err(CommandLineError::NumberOfParameters);
    }
    if !file_exists(&args[0]) {
        return Err(CommandLineError::FileNotFound(args[1].clone()));
    }

    Ok(Png2RexArgs {
        input: args[1].clone(),
        output: args[2].clone(),
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
        let r = parse_args(&["flibble".to_string(), "flibble".to_string()]);
        assert!(r.is_err());
        if let Err(e) = r {
            assert_eq!(e, CommandLineError::FileNotFound("flibble".to_string()));
        }
    }

    #[test]
    fn valid() {
        assert!(parse_args(&["resources/kitty.png".to_string(), "kitty.rex".to_string()]).is_ok());
    }

    #[test]
    fn really_valid() {
        if let Ok(p) = parse_args(&["resources/kitty.png".to_string(), "kitty.xp".to_string()]) {
            assert_eq!(p.input, "resources/kitty.png");
            assert_eq!(p.output, "kitty.xp");
        }
    }
}
