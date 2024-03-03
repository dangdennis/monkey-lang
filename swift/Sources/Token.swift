enum TokenType {
    case illegal
    case eof

    // identifers + literals
    case ident
    case int

    // operators
    case assign
    case plus

    // delimiters
    case comma
    case semicolon
    case lParen
    case rParen
    case lBrace
    case rBrace

    // keywords
    case function
    case `let`
}

struct Token {
    let type: TokenType
    let literal: String?
}

let KEYWORDS = [
    "fn": TokenType.function,
    "let": TokenType.let
]

func lookupIdent(ident: String) -> TokenType? {
    if let type = KEYWORDS[ident] {
        return type
    }
    return nil
}