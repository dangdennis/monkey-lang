package monkey

enum TokenType:
  case Illegal
  case Eof

  // identifiers + literals
  case Ident
  case Int

  // operators
  case Assign
  case Plus
  case Minus
  case Bang
  case Asterisk
  case FSlash

  case Lt
  case Gt
  case Eq
  case NotEq

  // delimiters
  case Comma
  case Semicolon
  case LParen
  case RParen
  case LBrace
  case RBrace

  // keywords
  case Function
  case Let
  case True
  case False
  case If
  case Else
  case Return

case class Token(
    _type: TokenType,
    literal: Option[String]
)

def KEYWORDS = Map(
  "fn" -> TokenType.Function,
  "let" -> TokenType.Let,
  "true" -> TokenType.True,
  "false" -> TokenType.False,
  "if" -> TokenType.If,
  "else" -> TokenType.Else,
  "return" -> TokenType.Return
)

def lookupIdent(ident: String): Option[TokenType] =
  KEYWORDS.get(ident)
