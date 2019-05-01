package ast

import (
	"github.com/dangdennis/monkey-go/token"
)

// Node is a single unit in our AST
type Node interface {
	TokenLiteral() string
}

// Statement is the combo of Identifier and Expression
type Statement interface {
	Node
	statementNode()
}

// Expression is the evaluated value of a Statement
type Expression interface {
	Node
	expressionNode()
}

// Program is the root node of every AST our parser produces
type Program struct {
	Statements []Statement
}

// TokenLiteral will be used only for debugging and testing
func (p *Program) TokenLiteral() string {
	if len(p.Statements) > 0 {
		return p.Statements[0].TokenLiteral()
	}

	return ""
}

// LetStatement is a Monkey let statement
type LetStatement struct {
	Token token.Token // the token.LET token
	Name  *Identifier // the x in "let x = 5;"
	Value Expression  // the 5 in "let x = 5;"
}

func (ls *LetStatement) statementNode() {}

// TokenLiteral of a LetStatement is "let"
func (ls *LetStatement) TokenLiteral() string { return ls.Token.Literal }

// Identifier is the left hand side of a LetStatement
type Identifier struct {
	Token token.Token // the token.IDENT token
	Value string
}

func (i *Identifier) expressionNode() {}

// TokenLiteral of an Identifier is the user's variable name
func (i *Identifier) TokenLiteral() string { return i.Token.Literal }
