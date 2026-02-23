use crate::cli;
use crate::display;
use crate::parser;

use async_recursion::async_recursion;
use futures::future::join_all;
use indextree::{Arena, NodeId};
use std::fmt;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct Options {
    pub help_string: String,
    pub commands: Vec<String>,
}

impl From<cli::Args> for Options {
    fn from(args: cli::Args) -> Self {
        Options {
            help_string: args.help_string,
            commands: args.commands,
        }
    }
}

#[derive(Debug)]
pub enum Error {
    CommandNotFound(String),
    CommandFailed(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CommandNotFound(cmd) => write!(f, "command not found: {cmd}"),
            Error::CommandFailed(cmd, msg) => write!(f, "failed to execute '{cmd}': {msg}"),
        }
    }
}

#[derive(Debug)]
pub struct CommandNode {
    pub index: usize,
    pub command: String,
    #[allow(dead_code)]
    pub description: Option<String>,
}

pub async fn run(options: Options) -> Result<(), Error> {
    let arena = Arc::new(Mutex::new(Arena::new()));

    let root = {
        let mut arena_locked = arena.lock().await;
        arena_locked.new_node(CommandNode {
            index: 0,
            command: options.commands.join(" "),
            description: None,
        })
    };

    // Execute the root command first to verify it exists
    let command_output = exec_command(options.commands.clone(), &options.help_string).await?;

    if let Some(commands) = parser::parse(&command_output) {
        if let Some(node_ids) = grow_subcommand_tree(
            Arc::clone(&arena),
            options.commands,
            options.help_string,
            commands,
        )
        .await
        {
            let mut arena_locked = arena.lock().await;
            for node_id in node_ids {
                root.append(node_id, &mut arena_locked)
            }
        }
    }

    let arena_locked = arena.lock().await;
    println!("{}", display::tree_string(&root, &arena_locked));
    Ok(())
}

#[async_recursion]
async fn grow_subcommand_tree(
    arena: Arc<Mutex<Arena<CommandNode>>>,
    command_path: Vec<String>,
    help_string: String,
    commands: Vec<(String, String)>,
) -> Option<Vec<NodeId>> {
    let futures: Vec<_> = commands
        .into_iter()
        .enumerate()
        .map(|(i, (cmd, desc))| {
            let command_path = command_path.clone();
            let help_string = help_string.clone();
            let arena = Arc::clone(&arena);

            async move {
                let mut command_path = command_path;
                command_path.push(cmd.clone());

                let node = {
                    let mut arena_locked = arena.lock().await;
                    arena_locked.new_node(CommandNode {
                        index: i,
                        command: cmd.clone(),
                        description: Some(desc.clone()),
                    })
                };

                if let Ok(output) = exec_command(command_path.clone(), &help_string).await {
                    if let Some(sub_commands) = parser::parse(&output) {
                        if let Some(child_node_ids) = grow_subcommand_tree(
                            Arc::clone(&arena),
                            command_path,
                            help_string,
                            sub_commands,
                        )
                        .await
                        {
                            let mut arena_locked = arena.lock().await;
                            for child_node in child_node_ids {
                                node.append(child_node, &mut arena_locked);
                            }
                        }
                    }
                }

                node
            }
        })
        .collect();

    let node_ids = join_all(futures).await;
    Some(node_ids)
}

async fn exec_command(mut commands: Vec<String>, help_string: &str) -> Result<String, Error> {
    commands.push(help_string.to_string());
    let output = Command::new(&commands[0])
        .args(&commands[1..])
        .output()
        .map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Error::CommandNotFound(commands[0].clone())
            } else {
                Error::CommandFailed(commands.join(" "), e.to_string())
            }
        })?;

    let output_str =
        std::str::from_utf8(&output.stdout).expect("Failed to convert output to string");

    Ok(output_str.to_string())
}
