use std::ffi::OsString;

use lexopt::{Arg, Error, Parser, ValueExt};

const DEFAULT_PORT: u16 = 14523;

/// The result of parsing the command-line arguments.
#[derive(Debug, Clone)]
pub enum Args {
    /// `rcp <SRC> <DEST>`
    ///
    /// Copies a file or directory from SRC to DEST.
    Copy { src: OsString, dest: OsString },
    /// `rcp [-p|--port PORT]`
    ///
    /// Starts the RCP server on the specified port.
    Serve { port: u16 },
}

impl Args {
    /// Parses the command-line arguments.
    pub fn parse() -> Result<Self, Error> {
        Self::new(std::env::args_os())
    }

    fn new(args: impl IntoIterator<Item = OsString>) -> Result<Self, Error> {
        let mut parser = Parser::from_iter(args);
        let mut src_and_dest: Option<(OsString, Option<OsString>)> = None;
        let mut port: Option<u16> = None;

        while let Some(arg) = parser.next()? {
            match arg {
                Arg::Short('p') | Arg::Long("port") => {
                    port = Some(parser.value()?.parse()?);
                }
                Arg::Value(val) => match src_and_dest {
                    None => src_and_dest = Some((val, None)),
                    Some((src, None)) => src_and_dest = Some((src, Some(val))),
                    Some(_) => return Err(Arg::Value(val).unexpected()),
                },
                _ => return Err(arg.unexpected()),
            }
        }

        match (src_and_dest, port) {
            (Some((src, Some(dest))), None) => Ok(Args::Copy { src, dest }),
            (None, Some(port)) => Ok(Args::Serve { port }),
            (None, None) => Ok(Args::Serve { port: DEFAULT_PORT }),
            (Some((_, None)), None) => Err(Error::MissingValue {
                option: Some("DEST".into()),
            }),
            (Some(_), Some(_)) => Err(Error::Custom(
                "Cannot specify both SRC/DEST and port".into(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A convenience macro to create an [`Args`]. A normal array requires
    /// all elements to be of the same type, but this macro allows mixing
    /// different [`Into<OsString>`] types.
    macro_rules! rcp {
        ($($arg:expr),* $(,)?) => {
            Args::new([::std::ffi::OsString::from("rcp") $(, ::std::ffi::OsString::from($arg))*])
        };
    }

    const PATHS: &[&str] = &[
        "file.txt",
        "/absolute/dir/",
        "/absolute/file",
        "relative/dir/",
        "./relative/../dir/",
        "relative/file",
        "./relative/../file",
    ];

    #[test]
    fn test_command_copy_ok() {
        for l in PATHS {
            for r in PATHS {
                match rcp!(l, r) {
                    Ok(Args::Copy { src, dest }) => {
                        assert_eq!(src, OsString::from(l));
                        assert_eq!(dest, OsString::from(r));
                    }
                    other => panic!("expected Ok(Args::Copy), got {:?}", other),
                }
            }
        }
    }

    #[test]
    fn test_command_serve_ok() {
        match rcp!() {
            Ok(Args::Serve { port }) => assert_eq!(port, DEFAULT_PORT),
            other => panic!("expected Ok(Args::Serve), got {:?}", other),
        }
        match rcp!("-p", "1234") {
            Ok(Args::Serve { port }) => assert_eq!(port, 1234),
            other => panic!("expected Ok(Args::Serve), got {:?}", other),
        }
        match rcp!("--port", "1234") {
            Ok(Args::Serve { port }) => assert_eq!(port, 1234),
            other => panic!("expected Ok(Args::Serve), got {:?}", other),
        }
    }

    #[test]
    fn test_command_copy_missing_dest() {
        match rcp!("src") {
            Err(Error::MissingValue { option: Some(opt) }) => assert_eq!(opt, "DEST"),
            other => panic!("expected Err(MissingValue DEST), got {:?}", other),
        }
    }

    #[test]
    fn test_command_conflicting_args() {
        match rcp!("src", "dest", "-p", "1234") {
            Err(Error::Custom(msg)) => {
                assert_eq!(msg.to_string(), "Cannot specify both SRC/DEST and port")
            }
            other => panic!("expected Err(Custom), got {:?}", other),
        }
    }
}
