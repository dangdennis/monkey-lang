package ast

import (
	"bytes"

	"github.com/dangdennis/monkey-go/token"
)

// Node is a single unit in our AST
type Node interface {
	TokenLiteral() string
	String() string
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

func (p *Program) String() string {
	var out bytes.Buffer
	for _, s := range p.Statements {
		out.WriteString(s.String())
	}

	return out.String()
}

// STATEMENTS

// LetStatement is a Monkey let statement
type LetStatement struct {
	Token token.Token // the token.LET token
	Name  *Identifier // the x in "let x = 5;"
	Value Expression  // the 5 in "let x = 5;"
}

func (ls *LetStatement) statementNode() {}

// TokenLiteral of a LetStatement is "let"
func (ls *LetStatement) TokenLiteral() string { return ls.Token.Literal }
func (ls *LetStatement) String() string {
	var out bytes.Buffer
	out.WriteString(ls.TokenLiteral() + " ")
	out.WriteString(ls.Name.String())
	out.WriteString(" = ")

	if ls.Value != nil {
		out.WriteString(ls.Value.String())
	}

	out.WriteString(";")

	return out.String()
}

// func (rs *ReturnStatement) String() string {
// 	var out bytes.Buffer
// 	out.WriteString(rs.TokenLiteral() + " ")
// 	if rs.ReturnValue != nil {
// 		out.WriteString(rs.ReturnValue.String())
// 	}
// 	out.WriteString(";")
// 	return out.String()
// }

// ExpressionStatement is a single expression without a let keyword
// i.e. x + 10; Most scripting languages have this
type ExpressionStatement struct {
	Token      token.Token // the first token of the expressoin
	Expression Expression
}

func (es *ExpressionStatement) statementNode() {}

// TokenLiteral completes the Statement interface
func (es *ExpressionStatement) TokenLiteral() string { return es.Token.Literal }
func (es *ExpressionStatement) String() string {
	if es.Expression != nil {
		return es.Expression.String()
	}

	return ""
}

// Identifier is the left hand side of a LetStatement
type Identifier struct {
	Token token.Token // the token.IDENT token
	Value string
}

func (i *Identifier) expressionNode() {}

// TokenLiteral of an Identifier is the user's variable name
func (i *Identifier) TokenLiteral() string { return i.Token.Literal }

func (i *Identifier) String() string { return i.Value }
