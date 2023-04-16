<h4 align="center">
   A CLI program to talk with Chat GPT, written in Rust.
</h4>

# GPT-CLI

## Setup

> Create a .env file containing your key at the root of the project

   ```sh
   echo "OPENAI_API_KEY=<your key>" > .env
   ```

## Usage
gpt-cli [OPTIONS]

Options:
  -t, --tokens <TOKENS>  The max number of tokens generated per message [default: 200]
  -m, --model <MODEL>    The model to be used [default: text-davinci-003]
  -h, --help             Print help
  -V, --version          Print version

## Example

   ```sh
   gpt-cli Linux: command to syncronize two folders -t 100
   ```
