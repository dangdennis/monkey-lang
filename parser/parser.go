package parser

import (
	"fmt"

	"github.com/dangdennis/monkey-go/ast"
	"github.com/dangdennis/monkey-go/lexer"
	"github.com/dangdennis/monkey-go/token"
)

// Parser is our Monkey parser!
type Parser struct {
	l         *lexer.Lexer
	errors    []string
	curToken  token.Token // Look to current token under examination, to decide what to do next
	peekToken token.Token // In case curToken does not given enough information
}

// New instantiates a Parser object with the result of a lexer
func New(l *lexer.Lexer) *Parser {
	p := &Parser{
		l:      l,
		errors: []string{},
	}
	// Read two tokens, so curTokens and peekToken are both set
	p.nextToken()
	p.nextToken()
	return p
}

// Errors is our getter for parser errors
func (p *Parser) Errors() []string {
	return p.errors
}

func (p *Parser) peekError(t token.TokenType) {
	msg := fmt.Sprintf("expected next token to be %s, got %s instead",
		t, p.peekToken.Type)

	p.errors = append(p.errors, msg)
}

// nextToken advances our pointer along the lexer
func (p *Parser) nextToken() {
	p.curToken = p.peekToken
	p.peekToken = p.l.NextToken()
}

// ParseProgram begins the parsing process
func (p *Parser) ParseProgram() *ast.Program {
	program := &ast.Program{}
	program.Statements = []ast.Statement{}

	for p.curToken.Type != token.EOF {
		stmt := p.parseStatement()
		if stmt != nil {
			program.Statements = append(program.Statements, stmt)
		}
		p.nextToken()
	}

	return program
}

// parseStatement is the root parsing evaluator
func (p *Parser) parseStatement() ast.Statement {
	switch p.curToken.Type {
	case token.LET:
		return p.parseLetStatement()
	default:
		return nil
	}
}

// parseLetStatment parses let statements lol
func (p *Parser) parseLetStatement() *ast.LetStatement {
	stmt := &ast.LetStatement{Token: p.curToken}

	if !p.expectPeek(token.IDENT) {
		return nil
	}

	stmt.Name = &ast.Identifier{Token: p.curToken, Value: p.curToken.Literal}

	if !p.expectPeek(token.ASSIGN) {
		return nil
	}

	// TODO: We're skipping the expressions until we
	// encounter a semicolon
	for !p.curTokenIs(token.SEMICOLON) {
		p.nextToken()
	}

	return stmt
}

// curTokenIs is a utility method that compares a token to the current token in the lexer
func (p *Parser) curTokenIs(t token.TokenType) bool {
	return p.curToken.Type == t
}

// curTokenIs is a utility method that compares a token to the next token in the lexer
func (p *Parser) peekTokenIs(t token.TokenType) bool {
	return p.peekToken.Type == t
}

// expectPeek asserts the identity of the next token in the lexer and advances it if true
func (p *Parser) expectPeek(t token.TokenType) bool {
	if p.peekTokenIs(t) {
		p.nextToken()
		return true
	}

	p.peekError(t)
	return false
}
