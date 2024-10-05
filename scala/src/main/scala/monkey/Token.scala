package monkey

enum TokenType(val literal: String):
  case Illegal extends TokenType("ILLEGAL")
  case Eof extends TokenType("EOF")

  // identifiers + literals
  case Ident extends TokenType("IDENT")
  case Int extends TokenType("INT")

  // operators
  case Assign extends TokenType("=")
  case Plus extends TokenType("+")
  case Minus extends TokenType("-")
  case Bang extends TokenType("!")
  case Asterisk extends TokenType("*")
  case FSlash extends TokenType("/")

  case Lt extends TokenType("<")
  case Gt extends TokenType(">")
  case Eq extends TokenType("==")
  case NotEq extends TokenType("!=")

  // delimiters
  case Comma extends TokenType(",")
  case Semicolon extends TokenType(";")
  case LParen extends TokenType("(")
  case RParen extends TokenType(")")
  case LBrace extends TokenType("{")
  case RBrace extends TokenType("}")

  // keywords
  case Function extends TokenType("FUNCTION")
  case Let extends TokenType("LET")
  case True extends TokenType("TRUE")
  case False extends TokenType("FALSE")
  case If extends TokenType("IF")
  case Else extends TokenType("ELSE")
  case Return extends TokenType("RETURN")

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
