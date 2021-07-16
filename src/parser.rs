use crate::Command;

fn parse_string(s: &str) -> Result<(String, usize), String> {
    if s.starts_with('"') {
        return match s[1..].find("\"") {
            Some(index) => Ok((String::from(&s[1..index + 1]), index + 2)),
            None => Err(String::from("Expected string termination")),
        };
    }

    match s.find(|c: char| c.eq(&' ') || c.eq(&'"')) {
        Some(index) => {
            if s[index..index + 1].eq("\"") {
                Err(String::from("Unexpected string initializer"))
            } else {
                Ok((String::from(&s[0..index]), index))
            }
        }
        None => Ok((String::from(s), s.len())),
    }
}

fn parse_args(mut args: &str) -> Result<Vec<String>, String> {
    let mut v = Vec::new();

    while let Some(index) = args.find(|c: char| !c.eq(&' ')) {
        args = &args[index..];
        let (arg, end) = parse_string(args)?;
        v.push(arg);
        if args[end..].is_empty() {
            break;
        }
        args = &args[end + 1..];
    }

    Ok(v)
}

fn parse_key_only(args: Vec<String>) -> Result<String, String> {
    match args.len() {
        0 => Err(String::from("Expected argument {key}")),
        1 => Ok(args[0].to_owned()),
        _ => Err(format!("Unexpected argument {}", args[1])),
    }
}

fn parse_key_value(args: Vec<String>) -> Result<(String, String), String> {
    match args.len() {
        0 => Err(String::from("Expected argument {key}")),
        1 => Err(String::from("Expected argument {value}")),
        2 => Ok((args[0].to_owned(), args[1].to_owned())),
        _ => Err(format!("Unexpected argument {}", args[2])),
    }
}

pub fn parse(s: &str) -> Result<Command, String> {
    let mut args = parse_args(s.trim())?;

    if args.is_empty() {
        return Err(String::from("Command not provided"));
    }

    let command = args.remove(0);

    match command.to_lowercase().as_str() {
        "get" => Ok(Command::Get(parse_key_only(args)?)),
        "set" => {
            let (key, value) = parse_key_value(args)?;
            Ok(Command::Set(key, value))
        }
        "del" => Ok(Command::Del(parse_key_only(args)?)),
        _ => Err(String::from("Invalid command")),
    }
}

#[cfg(test)]
mod test {
    use crate::Command;

    use super::parse;

    #[test]
    fn parses_basic_get_command() {
        assert_eq!(
            parse("get some_key"),
            Ok(Command::Get(String::from("some_key")))
        );

        assert_eq!(
            parse("     GET          some_key       "),
            Ok(Command::Get(String::from("some_key")))
        );
    }

    #[test]
    fn parses_get_command_with_string() {
        assert_eq!(
            parse("get \"some key\""),
            Ok(Command::Get(String::from("some key")))
        );

        assert_eq!(
            parse("get \"some    key  with spaces\""),
            Ok(Command::Get(String::from("some    key  with spaces")))
        );

        assert_eq!(
            parse("     GET          \"some key\"       "),
            Ok(Command::Get(String::from("some key")))
        );
    }

    #[test]
    fn returns_error_for_malformed_get_commands() {
        assert_eq!(
            parse("get \"some key"),
            Err(String::from("Expected string termination"))
        );

        assert_eq!(
            parse("get som\"e"),
            Err(String::from("Unexpected string initializer"))
        );

        assert_eq!(parse("get "), Err(String::from("Expected argument {key}")));

        assert_eq!(
            parse("get some_key some_value"),
            Err(String::from("Unexpected argument some_value"))
        );
    }

    #[test]
    fn returns_error_for_unknown_commands() {
        assert_eq!(parse(""), Err(String::from("Command not provided")));

        assert_eq!(parse("     "), Err(String::from("Command not provided")));

        assert_eq!(parse("unknown"), Err(String::from("Invalid command")));

        assert_eq!(
            parse("unknown some_key"),
            Err(String::from("Invalid command"))
        );
    }

    #[test]
    fn parses_basic_set_command() {
        assert_eq!(
            parse("set some_key some_value"),
            Ok(Command::Set(
                String::from("some_key"),
                String::from("some_value")
            ))
        );

        assert_eq!(
            parse("     SET          some_key    some_value   "),
            Ok(Command::Set(
                String::from("some_key"),
                String::from("some_value")
            ))
        );
    }

    #[test]
    fn parses_set_command_with_string() {
        assert_eq!(
            parse("set \"some key\" \"some value\""),
            Ok(Command::Set(
                String::from("some key"),
                String::from("some value")
            ))
        );

        assert_eq!(
            parse("SET    \"some key\"       \"some value\""),
            Ok(Command::Set(
                String::from("some key"),
                String::from("some value")
            ))
        );

        assert_eq!(
            parse("set \"some    key  with spaces\" \" some value with   spaces\"   "),
            Ok(Command::Set(
                String::from("some    key  with spaces"),
                String::from(" some value with   spaces")
            ))
        );
    }

    #[test]
    fn returns_error_for_malformed_set_commands() {
        assert_eq!(
            parse("set \"some key"),
            Err(String::from("Expected string termination"))
        );

        assert_eq!(
            parse("set \"some key\" \"some value  "),
            Err(String::from("Expected string termination"))
        );

        assert_eq!(
            parse("set som\"e"),
            Err(String::from("Unexpected string initializer"))
        );

        assert_eq!(
            parse("set some_key some\"value"),
            Err(String::from("Unexpected string initializer"))
        );

        assert_eq!(parse("set"), Err(String::from("Expected argument {key}")));

        assert_eq!(
            parse("set \"some key\""),
            Err(String::from("Expected argument {value}"))
        );
    }

    #[test]
    fn parses_basic_del_command() {
        assert_eq!(
            parse("del some_key"),
            Ok(Command::Del(String::from("some_key")))
        );
    }
}
