use crate::cli;
use crate::display;
use crate::parser;

use async_recursion::async_recursion;
use futures::future::join_all;
use indextree::{Arena, NodeId};
use std::process::Command;
use std::sync::Arc;
use tokio::sync::Mutex;
//use tokio::task::JoinSet;

#[derive(Debug, Clone)]
pub struct Options {
    pub help_string: String,
    pub commands: Vec<String>,
}

impl From<cli::Args> for Options {
    fn from(args: cli::Args) -> Self {
        Options {
            help_string: match args.help_string {
                Some(s) => s,
                None => "--help".to_string(),
            },
            commands: args.commands,
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CommandNode {
    pub index: usize,
    pub command: String,
    pub description: Option<String>,
}

pub async fn run(options: Options) {
    let arena = Arc::new(Mutex::new(Arena::new()));

    let root = {
        let mut arena_locked = arena.lock().await;
        arena_locked.new_node(CommandNode {
            index: 0,
            command: options.commands.join(" "),
            description: None,
        })
    };

    if let Some(node_ids) =
        grow_subcommand_tree(Arc::clone(&arena), options.commands, options.help_string).await
    {
        let mut arena_locked = arena.lock().await;
        for node_id in node_ids {
            root.append(node_id, &mut arena_locked)
        }
    }

    let arena_locked = arena.lock().await;
    println!("{}", display::tree_string(&root, &arena_locked));
}

#[async_recursion]
async fn grow_subcommand_tree(
    arena: Arc<Mutex<Arena<CommandNode>>>,
    command_path: Vec<String>,
    help_string: String,
) -> Option<Vec<NodeId>> {
    let command_output = exec_command(command_path.clone(), &help_string).await;

    if let Some(commands) = parser::parse(&command_output) {
        let futures: Vec<_> = commands
            .into_iter()
            .enumerate()
            .map(|(i, (cmd, desc))| {
                let command_path = command_path.clone();
                let help_string = help_string.clone();
                let arena = Arc::clone(&arena);

                async move {
                    let mut command_path = command_path.clone();
                    command_path.push(cmd.clone());

                    let node = {
                        let mut arena_locked = arena.lock().await;
                        arena_locked.new_node(CommandNode {
                            index: i,
                            command: cmd.clone(),
                            description: Some(desc.clone()),
                        })
                    };

                    if let Some(child_node_ids) =
                        grow_subcommand_tree(Arc::clone(&arena), command_path, help_string).await
                    {
                        let mut arena_locked = arena.lock().await;
                        for child_node in child_node_ids {
                            node.append(child_node, &mut arena_locked);
                        }
                    }

                    node
                }
            })
            .collect();

        let node_ids = join_all(futures).await;
        Some(node_ids)
    } else {
        None
    }
}

async fn exec_command(mut commands: Vec<String>, help_string: &str) -> String {
    commands.push(help_string.to_string());
    let output = Command::new(&commands[0])
        .args(&commands[1..])
        .output()
        .expect("Failed to execute command");

    let output_str =
        std::str::from_utf8(&output.stdout).expect("Failed to convert output to string");

    output_str.to_string()
}
