package monkey

@main def run(): Unit =
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
