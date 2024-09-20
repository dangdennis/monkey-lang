package monkey

@main def monkey() =
  println("Welcome to Monkey 0.1.0")
  val input = "=+poke(){},;"
  var lex = Lexer.apply(input)
  println("get next token")
  val (t1, l1) = Lexer.nextToken(lex)
  println("get next token 2")
  val (t2, l2) = Lexer.nextToken(l1)
  println("get next token 3")
  val (t3, l3) = Lexer.nextToken(l2)
  println(s"$t1")
  println(s"$t2")
  println(s"$t3")

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
            if StringUtils.isLetter(ch) then
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
  end nextToken

  private def readIdentifier(lexer: Lexer): String =
    val startingPos = lexer.position

    var l = lexer
    val currentChar = lexer.ch

    currentChar match
      case None => ()
      case Some(ch) =>
        var c = ch
        while StringUtils.isLetter(c) do
          println(s"$c")
          l = Lexer.readChar(l)
          println(l)

    val ident = lexer.input.substring(startingPos, l.readPosition)

    ident
  end readIdentifier

object StringUtils:
  def isLetter(ch: String): Boolean =
    "a" <= ch && ch <= "z" || "A" <= ch && ch <= "Z" || ch == "_" || ch == "?" || ch == "!"

end StringUtils
