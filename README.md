<h4 align="center">
    A simple, and efficient, CLI program to find a Linux command with chat GPT.
</h4>

![Made with VHS](https://vhs.charm.sh/vhs-2KmNYiklN8x8aaOEeM7okr.gif)

# GPT-CLI

## Setup

You need to have the var `OPENAI_KEY` exported from your environment. I recommend exporting it on `.zshrc` for zsh shells (or .bashrc for bash). Or you can also create a local .env at the root of the project.

> Create a .env file containing your key at the root of the project

   ```sh
   echo "export OPENAI_KEY=<your key>" > .env
   ```

## Usage
```sh
gpt-cli [OPTIONS] <PROMPT>

Arguments:
  <PROMPT>  Your prompt

Options:
  -t, --tokens           The max number of tokens generated per message
  -m, --model            The model to be used [default: text-davinci-003]
  -h, --help             Print help
  -V, --version          Print version

Example

   gpt-cli "syncronize two folders" -t 100
```

## Installation
The binary name is called `gpt`. In the future, the binaries will be added to the Arch Linux package manager, pacman. For now, you can download the compiled binaries on the [releases page](https://github.com/gabrieldemian/gpt-cli/releases), or build it from source.

### Building from source
gpt-cli is written in Rust, so you'll need to have a Rust installation in order to compile it.

To build gpt-cli:
```sh
$ git clone git@github.com:gabrieldemian/gpt-cli.git
$ cd gpt-cli
$ cargo build --release
$ ./target/release/gpt --help
```

Copy the binary to a PATH folder:
```sh
$ sudo cp ./target/release/gpt /usr/bin/gpt
```
