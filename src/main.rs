use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::process::{Command, Output};
#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct ToolsList {
    tools: Vec<Tools>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
struct Tools {
    name: String,
    path: String,
    git: String,
    default_args: Vec<String>,
}
#[derive(Debug, PartialEq, Parser)]
struct MainCommand {
    tool: String,
    #[arg(short, long, required = false)]
    update: bool,
    //#[arg(required = false)]
    //list:bool,
    #[arg(
        required = false,
        default_value = "",
        allow_hyphen_values = true,
        value_delimiter = ','
    )]
    args: Vec<String>,
    #[arg(
        required = false,
        default_value = "./.tanuki-tools.yaml",
        allow_hyphen_values = true,
        env,
        last(true)
    )]
    tanuki_conf_path: String,
}
fn load_config(path: &str) -> Result<ToolsList, serde_yaml::Error> {
    let file = File::open(path).expect(&format!("File not found: \"{}\" :)", path));
    let tools: Result<_, _> = serde_yaml::from_reader::<File, ToolsList>(file);
    Ok(tools?)
}
fn search_tools_name(tools_list: &ToolsList, name: &str) -> (bool, usize) {
    for (index, tools) in tools_list.tools.iter().enumerate() {
        if tools.name == name.to_string() {
            return (true, index);
        }
    }
    return (false, 0);
}
fn print_command_output(output: &Output) {
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}
fn git_clone(git_path: &str) {
    let out = Command::new("git")
        .arg("clone")
        .arg(git_path)
        .output()
        .expect("");
    print_command_output(&out);
}

fn exec(command: &mut MainCommand) {
    let tools_list: ToolsList = load_config(&command.tanuki_conf_path).unwrap();
    let (res, index) = search_tools_name(&tools_list, &command.tool);
    let path: String = tools_list.tools[index].path.clone();
    let default_args: Vec<String> = tools_list.tools[index].default_args.clone();
    let update = command.update;
    let list = command.list;
    let git_path = tools_list.tools[index].git.clone();
    let args: Vec<String> = command.args.clone();
    //if list {
        //for tools in tools_list.tools{
            //println!("tools list: {:?}",tools.name);
        //}
        //return;
    //}
    if update {
        git_clone(&git_path);
        //if let Err(e) = env::set_current_dir(
        //let out =
    } else {
    }
    if res {
        let mut cmd = Command::new(&path);
        if default_args.is_empty() {
            if !args.is_empty() {
                let out = cmd.args(&args).output().expect("");
                print_command_output(&out);
            }
        } else {
            if !args.is_empty() {
                let out = cmd.args(&args).args(&default_args).output().expect("");
                print_command_output(&out);
            } else {
                let out = cmd.args(&default_args).output().expect("");
                print_command_output(&out);
            }
        }
    } else {
        panic!("Error! not found tools: {}", command.tool);
    }
    //println!("{:?}",tools_list);
    //println!("{:?}",command);
}
fn main() -> Result<(), serde_yaml::Error> {
    let mut main_command = MainCommand::parse();
    exec(&mut main_command);
    Ok(())
}
