use clap::{ArgGroup, Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Parser)]
struct List {}
#[derive(Debug, Subcommand)]
enum MainSubCommand {
    List(List),
}

#[derive(Debug, Parser)]
struct MainCommand {
    #[clap(subcommand)]
    subcommand: Option<MainSubCommand>,
    #[arg(required = false, default_value = "")]
    tool: String,
    #[arg(short, long, required = false)]
    update: bool,
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
fn git_path_to_folder(git_path: &str) -> String {
    let pattern = "https://github.com/";

    let part: Vec<&str> = git_path.split(pattern).collect();
    let mut user_name = String::new();
    let mut folder_name = String::new();
    if let Some(uname) = part.get(1) {
        user_name = uname.to_string();
        let part: Vec<&str> = git_path.split(&user_name).collect();
        if let Some(fname) = part.get(1) {
            folder_name = fname.to_string();
        }
    }

    folder_name
}
fn print_tools_list(tools_list: &ToolsList) {
    println!("-- Current registered tools --");
    for tools in &tools_list.tools {
        println!("name: {}", tools.name);
    }
}
fn exec(command: &MainCommand) -> Result<(), serde_yaml::Error> {
    let tools_list: ToolsList = load_config(&command.tanuki_conf_path)?;
    let (res, index) = search_tools_name(&tools_list, &command.tool);
    let path: String = tools_list.tools[index].path.clone();
    let default_args: Vec<String> = tools_list.tools[index].default_args.clone();
    let update = command.update;
    let git_path = tools_list.tools[index].git.clone();
    let args: Vec<String> = command.args.clone();
    match &command.subcommand {
        Some(sub) => match sub {
            MainSubCommand::List(_) => {
                print_tools_list(&tools_list);
                return Ok(());
            }
        },
        None => {}
    }
    // 未実装
    /*
    if update {
        git_clone(&git_path);
        let git_folder = git_path_to_folder(&git_path);
        println!("git_folder: {}",git_folder);
        if let Err(e) = env::set_current_dir(&git_folder){
            println!("{}",e);
        }else{
            let mut build_cmd = Command::new("cargo");
            let build_out = build_cmd.arg("build").arg("--release").output().expect("not found `cargo` command");
            let bin_path:String = format!("{}/{}","target/release",tools_list.tools[index].name.clone());
            let mut cp_cmd = Command::new("cp");
            let cp_out = cp_cmd.arg(&bin_path).arg(".").output().expect("not found `cp` command");
            let mut del_cmd = Command::new("sudo");
            let del_out =del_cmd.arg("rm").arg("-r").arg(git_folder).output().expect("not found `rm` command");
        }

    }
    */
    if res {
        let mut cmd = Command::new(&path);
        if default_args.is_empty() {
            if !args.is_empty() {
                let out = cmd.args(&args).output().expect("");
                print_command_output(&out);
            }
        } else {
            if !args.is_empty() {
                let out = cmd.args(&default_args).args(&args).output().expect("");
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

    Ok(())
}
fn main() -> Result<(), serde_yaml::Error> {
    let main_command = MainCommand::parse();
    exec(&main_command)?;
    Ok(())
}
