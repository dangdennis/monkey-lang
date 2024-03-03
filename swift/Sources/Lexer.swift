struct Lexer {
  let input: String
  var position: Int
  var readPosition: Int
  var ch: Character?
}

extension Lexer {
  init(input: String) {
    self.input = input
    self.position = 0
    self.readPosition = 0
    self.ch = input.first

    self.readChar()
  }

  mutating func readChar() {
    if readPosition >= input.count {
      ch = nil
    } else {
      ch = input[input.index(input.startIndex, offsetBy: readPosition)]
    }
    position = readPosition
    readPosition += 1
  }

  mutating func nextToken() -> Token {
    let (tokenType, literal): (TokenType, String?) =
      switch ch {
      case "=": (.assign, "=")
      case ";": (.semicolon, ";")
      case "(": (.lParen, "(")
      case ")": (.rParen, ")")
      case ",": (.comma, ",")
      case "+": (.plus, "+")
      case "{": (.lBrace, "{")
      case "}": (.rBrace, "}")
      default:

        (.illegal, nil)
      }

    self.readChar()

    return tok
  }
}

let lexer = Lexer(input: "dsfdfsdf")
