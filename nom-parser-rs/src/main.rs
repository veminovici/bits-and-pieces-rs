use nom::character::complete::digit1;
use nom::error::*;
use nom::Err;
use nom::IResult;

fn parser(input: &str) -> IResult<&str, &str> {
    digit1(input)
}

fn main() {
    assert_eq!(parser("21c"), Ok(("c", "21")));
    assert_eq!(
        parser("c1"),
        Err(Err::Error(Error::new("c1", ErrorKind::Digit)))
    );
    assert_eq!(
        parser(""),
        Err(Err::Error(Error::new("", ErrorKind::Digit)))
    );
}
