open Monkey

let test_next_token () =
  let input =
    {|
let five = 5;
let ten = 10;
let add = fn(x, y) {
  x + y;
};
let result = add(five, ten);
|}
  in
  let l = Lexer.make input in
  let test_cases =
    [
      (Token.Let, "let");
      (Ident, "five");
      (Assign, "=");
      (Int, "5");
      (Semicolon, ";");
      (Let, "let");
      (Ident, "ten");
      (Assign, "=");
      (Int, "10");
      (Semicolon, ";");
      (Let, "let");
      (Ident, "add");
      (Assign, "=");
      (Function, "fn");
      (LParen, "(");
      (Ident, "x");
      (Comma, ",");
      (Ident, "y");
      (RParen, ")");
      (LBrace, "{");
      (Ident, "x");
      (Plus, "+");
      (Ident, "y");
      (Semicolon, ";");
      (RBrace, "}");
      (Semicolon, ";");
      (Let, "let");
      (Ident, "result");
      (Assign, "=");
      (Ident, "add");
      (LParen, "(");
      (Ident, "five");
      (Comma, ",");
      (Ident, "ten");
      (RParen, ")");
      (Semicolon, ";");
      (Eof, "");
    ]
  in

  let _ =
    List.fold_left
      (fun lexer (_expected_token, expected_lit) ->
        let (lexer, next_token) = Lexer.next_token lexer in
        Alcotest.(check string) "expect correct token" next_token.literal expected_lit;
        lexer)
      l test_cases
  in

