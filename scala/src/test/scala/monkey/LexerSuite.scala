package monkey

class LexerSuite extends munit.FunSuite:
  test("next token 1"):
    val input = "=+(){},;"

    val tests = List(
      (TokenType.Assign, Some("=")),
      (TokenType.Plus, Some("+")),
      (TokenType.LParen, Some("(")),
      (TokenType.RParen, Some(")")),
      (TokenType.LBrace, Some("{")),
      (TokenType.RBrace, Some("}")),
      (TokenType.Comma, Some(",")),
      (TokenType.Semicolon, Some(";")),
      (TokenType.Eof, None)
    )

    tests.zipWithIndex.foldLeft(Lexer(input)) {
      case (currentLexer, ((expectedType, expectedLiteral), i)) =>
        val (token, lexer) = Lexer.nextToken(currentLexer)

        assertEquals(token._type, expectedType, s"tests[$i] - tokentype wrong")
        assertEquals(
          token.literal,
          expectedLiteral,
          s"tests[$i] - literal wrong"
        )

        lexer
    }

  test("next token 2"):
    val input = """
    let five = 5;
    let ten = 10;
    let add = fn(x, y) {
      x + y;
    };
    """

    val tests = List(
      (TokenType.Let, Some("let")),
      (TokenType.Ident, Some("five")),
      (TokenType.Assign, Some("=")),
      (TokenType.Int, Some("5")),
      (TokenType.Semicolon, Some(";")),
      (TokenType.Let, Some("let")),
      (TokenType.Ident, Some("ten")),
      (TokenType.Assign, Some("=")),
      (TokenType.Int, Some("10")),
      (TokenType.Semicolon, Some(";")),
      (TokenType.Let, Some("let")),
      (TokenType.Ident, Some("add")),
      (TokenType.Assign, Some("=")),
      (TokenType.Function, Some("fn")),
      (TokenType.LParen, Some("(")),
      (TokenType.Ident, Some("x")),
      (TokenType.Comma, Some(",")),
      (TokenType.Ident, Some("y")),
      (TokenType.RParen, Some(")")),
      (TokenType.LBrace, Some("{")),
      (TokenType.Ident, Some("x")),
      (TokenType.Plus, Some("+")),
      (TokenType.Ident, Some("y")),
      (TokenType.Semicolon, Some(";")),
      (TokenType.RBrace, Some("}")),
      (TokenType.Semicolon, Some(";")),
      (TokenType.Eof, None)
    )

    tests.zipWithIndex.foldLeft(Lexer(input)) {
      case (currentLexer, ((expectedType, expectedLiteral), i)) =>
        val (token, lexer) = Lexer.nextToken(currentLexer)

        assertEquals(token._type, expectedType, s"tests[$i] - tokentype wrong")
        assertEquals(
          token.literal,
          expectedLiteral,
          s"tests[$i] - literal wrong"
        )

        lexer
    }

end LexerSuite
