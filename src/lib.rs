use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    Assistant,
    User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub text: Option<String>,
    pub index: u32,
    pub logprobs: Option<String>,
    pub finish_reason: Option<String>,
    pub message: Option<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionBody {
    pub model: String,
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletionResp {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: u64,
    pub model: Option<String>,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

/// A simple, and efficient, CLI program to find a Linux command with chat GPT.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Your prompt
    #[arg()]
    pub prompt: String,

    /// The max number of tokens generated per message
    #[arg(short, long)]
    pub tokens: Option<i32>,

    /// The model to be used
    #[arg(short, long, default_value = "text-davinci-003")]
    pub model: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_find_command_in_string() {
        let teste = "The Linux command to synchronize two folders using `rsync` is:\n\n```\nrsync -avh /path/to/source/folder/ /path/to/destination/folder/\n```\n\nNote that this command will copy all files and folders from the source folder to the destination folder, and any files or folders that exist in the destination folder but not in the source folder will be deleted. If you want to sync two folders over the network, you'll need to specify the hostname and username for the remote machine.\ncode of the linux command to syncronize two folders\n\nHere's the code for the Linux command to synchronize two folders using `rsync`:\n\n```\nrsync -avh /path/to/source/folder/ /path/to/destination/folder/\n```\n\nThis command will copy all files and folders from the source folder to the destination folder, and any files or folders that exist in the destination folder but not in the source folder will be deleted. The `-a` option specifies that `rsync` should preserve all file attributes, including permissions, timestamps, and ownership, and the `-v` option enables verbose output so you can see what `rsync` is doing. The `-h` option is optional, and it stands for \"human-readable.\" If you include this option, `rsync` will display file sizes in a more easily readable format.";

        let l = teste.lines().position(|x| x == "```");
        let command = teste.lines().nth(l.unwrap() + 1).unwrap().trim();
        assert_eq!(
            command,
            "rsync -avh /path/to/source/folder/ /path/to/destination/folder/"
        );
    }
}
