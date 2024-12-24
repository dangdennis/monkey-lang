use monkey::token::{lookup_ident, TokenType};

#[test]
fn test_lookup_ident() {
    assert_eq!(lookup_ident("fn"), TokenType::Function);
    assert_eq!(lookup_ident("let"), TokenType::Let);
    assert_eq!(lookup_ident("foobar"), TokenType::Ident);
}
