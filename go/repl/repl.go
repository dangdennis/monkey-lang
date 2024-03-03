package repl

import (
	"bufio"
	"fmt"
	"io"
	"github.com/dangdennis/monkey-go/lexer"
	"github.com/dangdennis/monkey-go/token"
)

// PROMPT signals user for input
const PROMPT = ">> "

// Start begins the Monkey REPL
func Start(in io.Reader, out io.Writer) {
	scanner := bufio.NewScanner(in)

	for {
		fmt.Printf(PROMPT)
		scanned := scanner.Scan()
		if !scanned {
			return
		}
		line := scanner.Text()
		l := lexer.New(line)

		for tok := l.NextToken(); tok.Type != token.EOF; tok = l.NextToken() {
			fmt.Printf("%+v\n", tok)
		}
	}
}
