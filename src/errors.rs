use error_chain::error_chain;

#[derive(Debug)]
pub struct NoneError {}

impl From<std::option::NoneError> for NoneError {
  fn from(_: std::option::NoneError) -> Self {
    Self{}
  }
}

impl std::error::Error for NoneError {

}

impl std::fmt::Display for NoneError {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    f.write_fmt(format_args!("{:?}", self))
  }
}

impl From<std::option::NoneError> for Error {
  fn from(s: std::option::NoneError) -> Self {
    ErrorKind::NoneError(s.into()).into()
  }
}

error_chain! {
    // The type defined for this error. These are the conventional
    // and recommended names, but they can be arbitrarily chosen.
    //
    // It is also possible to leave this section out entirely, or
    // leave it empty, and these names will be used automatically.
    types {
      Error, ErrorKind, ResultExt, Result;
    }

    // Without the `Result` wrapper:
    //
    // types {
    //     Error, ErrorKind, ResultExt;
    // }

    // Automatic conversions between this error chain and other
    // error chains. In this case, it will e.g. generate an
    // `ErrorKind` variant called `Another` which in turn contains
    // the `other_error::ErrorKind`, with conversions from
    // `other_error::Error`.
    //
    // Optionally, some attributes can be added to a variant.
    //
    // This section can be empty.
    links {
    }

    // Automatic conversions between this error chain and other
    // error types not defined by the `error_chain!`. These will be
    // wrapped in a new error with, in the first case, the
    // `ErrorKind::Fmt` variant. The description and cause will
    // forward to the description and cause of the original error.
    //
    // Optionally, some attributes can be added to a variant.
    //
    // This section can be empty.
    foreign_links {
      Fmt(::std::fmt::Error);
      Io(::std::io::Error);
      NoneError(NoneError);
      Infallible(std::convert::Infallible);
      Decimal(rust_decimal::Error);
      Reqwest(reqwest::Error);
      ParseInt(std::num::ParseIntError);
      UTF8(std::string::FromUtf8Error);
      Yaml(serde_yaml::Error);
      LogLevel(log::SetLoggerError);
      LogLevelParse(log::ParseLevelError);
      TimeError(std::time::SystemTimeError);
    }

    // Define additional `ErrorKind` variants.  Define custom responses with the
    // `description` and `display` calls.
    errors {
      NoConfigFound {
        description("configuration file not found"),
        display("configuration file not found")
      }

      UnknownWebhookType(s: String) {
        description("webhook type is unknown"),
        display("webhook type {} is unknown", s)
      }
    }
}