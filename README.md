<h4 align="center">
    A simple, and efficient, CLI program to find a Linux command with chat GPT.
</h4>

# GPT-CLI

## Setup

You need to have the var `OPENAI_API_KEY` exported from your environment. I recommend exporting it on `.zshrc` for zsh shells (or .bashrc for bash). Or you can also create a local .env at the root of the project.

> Create a .env file containing your key at the root of the project

   ```sh
   echo "export OPENAI_API_KEY=<your key>" > .env
   ```

## Usage
<pre>
gpt-cli [OPTIONS] <PROMPT>

Arguments:
  <PROMPT>  Your prompt

Options:
  -t, --tokens           The max number of tokens generated per message
  -m, --model            The model to be used [default: text-davinci-003]
  -h, --help             Print help
  -V, --version          Print version
</pre>

## Example

   ```sh
   gpt-cli "syncronize two folders" -t 100
   ```