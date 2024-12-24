use monkey::lexer::Lexer;
use monkey::token::TokenType;

#[test]
fn test_next_token_simple() {
    let input = "=+(){},;";

    #[derive(Debug)]
    struct Test {
        expected_type: TokenType,
        expected_literal: Option<String>,
    }

    let tests = [
        Test {
            expected_type: TokenType::Assign,
            expected_literal: Some("=".to_string()),
        },
        Test {
            expected_type: TokenType::Plus,
            expected_literal: Some("+".to_string()),
        },
        Test {
            expected_type: TokenType::LParen,
            expected_literal: Some("(".to_string()),
        },
        Test {
            expected_type: TokenType::RParen,
            expected_literal: Some(")".to_string()),
        },
        Test {
            expected_type: TokenType::LBrace,
            expected_literal: Some("{".to_string()),
        },
        Test {
            expected_type: TokenType::RBrace,
            expected_literal: Some("}".to_string()),
        },
        Test {
            expected_type: TokenType::Comma,
            expected_literal: Some(",".to_string()),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: Some(";".to_string()),
        },
    ];

    let mut lexer = Lexer::new(input.to_string());

    for test in tests.iter() {
        let token = lexer.next_token();
        assert_eq!(token.token_type, test.expected_type);
        assert_eq!(token.literal, test.expected_literal);
    }
}

#[test]
fn test_next_token_syntax() {
    let input = r#"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
        "#;

    #[derive(Debug)]
    struct Test {
        expected_type: TokenType,
        expected_literal: Option<String>,
    }

    let tests = [
        Test {
            expected_type: TokenType::Let,
            expected_literal: Some("let".to_string()),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: Some("five".to_string()),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: Some("=".to_string()),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: Some("5".to_string()),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: Some(";".to_string()),
        },
        Test {
            expected_type: TokenType::Let,
            expected_literal: Some("let".to_string()),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: Some("ten".to_string()),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: Some("=".to_string()),
        },
        Test {
            expected_type: TokenType::Int,
            expected_literal: Some("10".to_string()),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: Some(";".to_string()),
        },
        Test {
            expected_type: TokenType::Let,
            expected_literal: Some("let".to_string()),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: Some("add".to_string()),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: Some("=".to_string()),
        },
        Test {
            expected_type: TokenType::Function,
            expected_literal: Some("fn".to_string()),
        },
        Test {
            expected_type: TokenType::LParen,
            expected_literal: Some("(".to_string()),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: Some("x".to_string()),
        },
        Test {
            expected_type: TokenType::Comma,
            expected_literal: Some(",".to_string()),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: Some("y".to_string()),
        },
        Test {
            expected_type: TokenType::RParen,
            expected_literal: Some(")".to_string()),
        },
        Test {
            expected_type: TokenType::LBrace,
            expected_literal: Some("{".to_string()),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: Some("x".to_string()),
        },
        Test {
            expected_type: TokenType::Plus,
            expected_literal: Some("+".to_string()),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: Some("y".to_string()),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: Some(";".to_string()),
        },
        Test {
            expected_type: TokenType::RBrace,
            expected_literal: Some("}".to_string()),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: Some(";".to_string()),
        },
        Test {
            expected_type: TokenType::Let,
            expected_literal: Some("let".to_string()),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: Some("result".to_string()),
        },
        Test {
            expected_type: TokenType::Assign,
            expected_literal: Some("=".to_string()),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: Some("add".to_string()),
        },
        Test {
            expected_type: TokenType::LParen,
            expected_literal: Some("(".to_string()),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: Some("five".to_string()),
        },
        Test {
            expected_type: TokenType::Comma,
            expected_literal: Some(",".to_string()),
        },
        Test {
            expected_type: TokenType::Ident,
            expected_literal: Some("ten".to_string()),
        },
        Test {
            expected_type: TokenType::RParen,
            expected_literal: Some(")".to_string()),
        },
        Test {
            expected_type: TokenType::Semicolon,
            expected_literal: Some(";".to_string()),
        },
        Test {
            expected_type: TokenType::Eof,
            expected_literal: None,
        },
    ];

    let mut lexer = Lexer::new(input.to_string());
    for test in tests.iter() {
        let token = lexer.next_token();
        assert_eq!(
            token.literal, test.expected_literal,
            "Expected literal {:?}, got {:?}",
            test.expected_literal, token.literal
        );
        assert_eq!(
            token.token_type, test.expected_type,
            "Expected token type {:?}, got {:?}",
            test.expected_type, token.token_type
        );
    }
}
