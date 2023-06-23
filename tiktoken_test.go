package tiktoken

import "testing"

func TestHelloToMyName(t *testing.T) {
	input := "Alexandre"
	expectedOutput := "Hello, Alexandre"
	output := HelloToMyName(input)

	if output != expectedOutput {
		t.Errorf("Got %s, expected %s", output, expectedOutput)
	}
}

func TestCountTokens(t *testing.T) {
	inputModel := "gpt-4"
	inputText := "Hello, my name is Alexandre"
	expectedOutput := 6
	output := CountTokens(inputModel, inputText)

	if output != expectedOutput {
		t.Errorf("Got %d, expected %d", output, expectedOutput)
	}
}
