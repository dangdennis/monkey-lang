open Base

type t = {
  input : string;
  position : int;
  read_position : int;
  ch : string option;
}

let read_char lexer =
  let new_char =
    if lexer.read_position > String.length lexer.input then None
    else Some (String.lexer.input lexer.read_position)
  in

  {
    lexer with
    position = lexer.read_position;
    read_position = lexer.read_position + 1;
    ch = new_char;
  }

let make input = read_char { input; position = 0; read_position = 0; ch = None }

let next_token lexer =
  match lexer.ch with
  | None -> (lexer, { Token.token_type = Eof; literal = "\000" })
  | Some ch ->
      let token_type =
        match ch with
        | "=" -> Token.Assign
        | ";" -> Semicolon
        | "(" -> LParen
        | ")" -> RParen
        | "," -> Comma
        | "+" -> Plus
        | "{" -> LBrace
        | "}" -> RBrace
        | _ -> Eof
      in
      let lexer = read_char lexer in
      (lexer, Token.make token_type ch)

(* let%expect_test "addition" =
     Stdio.printf "%d" (1 + 2);
     [%expect {| 3 |}]

   let%test_module "next_token" =
     (module struct
       let input = "=+(){},;" in
       let tokens =
         [
           { Token.token_type = Assign; literal = '=' };
           { token_type = Plus; literal = '+' };
           { token_type = LParen; literal = '(' };
           { token_type = RParen; literal = ')' };
           { token_type = LBrace; literal = '{' };
           { token_type = RBrace; literal = '}' };
           { token_type = Comma; literal = ',' };
           { token_type = Semicolon; literal = ';' };
         ]
       in

       let lexer = init input in

       let _ =
         List.fold tokens ~init:lexer ~f:(fun lexer expected_token ->
             let lexer, token = next_token lexer in
             (* assert (phys_equal token.token_type expected_token.token_type); *)
             assert (Char.equal token.literal expected_token.literal);
             lexer)
       in
       ()
     end) *)

(* let%expect_test "next_token" =
   let input = "=+(){},;" in
   let tokens =
     [
       { Token.token_type = Assign; literal = '=' };
       { token_type = Plus; literal = '+' };
       { token_type = LParen; literal = '(' };
       { token_type = RParen; literal = ')' };
       { token_type = LBrace; literal = '{' };
       { token_type = RBrace; literal = '}' };
       { token_type = Comma; literal = ',' };
       { token_type = Semicolon; literal = ';' };
     ]
   in

   let lexer = init input in

   let _ =
     List.fold tokens ~init:lexer ~f:(fun lexer _expected_token ->
         let lexer, token = next_token lexer in
         Stdio.printf "%c" token.literal;
         [%expect {| |}];
         lexer)
   in
   () *)
