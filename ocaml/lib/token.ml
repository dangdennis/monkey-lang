type token_type =
  | Illegal
  | Eof
  (* Identifiers + literals *)
  | Ident
  | Int
  (* Operators *)
  | Assign
  | Plus
  (* Delimiters *)
  | Comma
  | Semicolon
  | LParen
  | RParen
  | LBrace
  | RBrace
  (* Keywords *)
  | Function
  | Let

type token = { token_type : token_type; literal : string }

let make typ literal = { token_type = typ; literal }
