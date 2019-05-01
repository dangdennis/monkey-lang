package parser

import (
	"github.com/dangdennis/monkey-go/ast"
	"github.com/dangdennis/monkey-go/lexer"
	"github.com/dangdennis/monkey-go/token"
)

// Parser is our Monkey parser!
type Parser struct {
	l *lexer.Lexer

	curToken  token.Token // Look to current token under examination, to decide what to do next
	peekToken token.Token // In case curToken does not given enough information
}

// New instantiates a Parser object with the result of a lexer
func New(l *lexer.Lexer) *Parser {
	p := &Parser{l: l}

	// Read two tokens, so curTokens and peekToken are both set
	p.nextToken()
	p.nextToken()

	return p
}

func (p *Parser) nextToken() {
	p.curToken = p.peekToken
	p.peekToken = p.l.NextToken()
}

// ParseProgram begins the parsing process
func (p *Parser) ParseProgram() *ast.Program {
	return nil
}
