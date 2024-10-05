package monkey

case class Lexer(
    input: String,
    position: Int,
    readPosition: Int,
    ch: Option[String]
)

object Lexer:
  def apply(input: String): Lexer =
    readChar(
      Lexer(
        input = input,
        position = 0,
        readPosition = 0,
        ch = None
      )
    )

  def readChar(lexer: Lexer): Lexer =
    val ch =
      if lexer.readPosition >= lexer.input.length() then None
      else Option(lexer.input(lexer.readPosition).toString())

    Lexer(
      input = lexer.input,
      position = lexer.readPosition,
      readPosition = lexer.readPosition + 1,
      ch = ch
    )

  def nextToken(lexer: Lexer): (Token, Lexer) =
    val (tokenType, literal) =
      lexer.ch.fold(TokenType.Eof, None: Option[String]): ch =>
        ch match
          case "=" => (TokenType.Assign, Some("="))
          case ";" => (TokenType.Semicolon, Some(";"))
          case "(" => (TokenType.LParen, Some("("))
          case ")" => (TokenType.RParen, Some(")"))
          case "," => (TokenType.Comma, Some(","))
          case "+" => (TokenType.Plus, Some("+"))
          case "{" => (TokenType.LBrace, Some("{"))
          case "}" => (TokenType.RBrace, Some("}"))
          case _ if isLetter(ch) =>
            (TokenType.Ident, Some(readIdentifier(lexer)))
          case _ => (TokenType.Illegal, None)

    (Token(_type = tokenType, literal = literal), readChar(lexer))

  private def readIdentifier(lexer: Lexer): String =
    lexer.input.drop(lexer.position).takeWhile(ch => isLetter(ch.toString))

  def isLetter(ch: String): Boolean =
    ch.matches("[a-zA-Z_?!]")
