use lalrpop_util::lalrpop_mod;

lalrpop_mod!(select); // synthesized by LALRPOP
lalrpop_mod!(calculator); // synthesized by LALRPOP

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculator() {
        assert!(calculator::TermParser::new().parse("22").is_ok());
        assert!(calculator::TermParser::new().parse("(22)").is_ok());
        assert!(calculator::TermParser::new().parse("((((22))))").is_ok());
        assert!(calculator::TermParser::new().parse("((22)").is_err());
    }
}
