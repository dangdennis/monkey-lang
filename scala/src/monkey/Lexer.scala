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

    lexer.copy(
      ch = ch,
      position = lexer.readPosition,
      readPosition = lexer.readPosition + 1
    )

  def nextToken(lexer: Lexer): (Token, Lexer) =
    val (tokenType, literal) = lexer.ch match
      case None => (TokenType.Eof, None)
      case Some(ch) =>
        ch match
          case "=" => (TokenType.Assign, Some("="))
          case ";" => (TokenType.Semicolon, Some(";"))
          case "(" => (TokenType.LParen, Some("("))
          case ")" => (TokenType.RParen, Some(")"))
          case "," => (TokenType.Comma, Some(","))
          case "+" => (TokenType.Plus, Some("+"))
          case "{" => (TokenType.LBrace, Some("{"))
          case "}" => (TokenType.RBrace, Some("}"))
          case _ =>
            if isLetter(ch) then
              println("found identifier")
              (TokenType.Ident, Some(Lexer.readIdentifier(lexer)))
            else (TokenType.Illegal, None)

    (
      Token(
        _type = tokenType,
        literal = literal
      ),
      Lexer.readChar(lexer)
    )

  private def readIdentifier(lexer: Lexer): String =
    val startingPos = lexer.position

    var l = lexer
    val currentChar = lexer.ch

    currentChar match
      case None => ()
      case Some(ch) =>
        var c = ch
        while isLetter(c) do println(s"$c")
        l = Lexer.readChar(l)
        println(l)

    val ident = lexer.input.substring(startingPos, l.readPosition)

    ident

  def isLetter(ch: String): Boolean =
    "a" <= ch && ch <= "z" || "A" <= ch && ch <= "Z" || ch == "_" || ch == "?" || ch == "!"
