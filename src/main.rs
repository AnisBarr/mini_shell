use std::io::{self,Write};
use std::process::{Command};
use std::path::Path;
use std::env;
use std::env::set_current_dir;
use std::collections::HashMap;





fn cmd_prossing(cmd : String){
    let list_cmd = space_split(cmd.as_str());

    let filtered_env : HashMap<String, String> =  env::vars().filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH").collect();

    if list_cmd[0].as_str() == "cd" {
        cd(list_cmd);
    }
    if list_cmd[0].as_str() == "back" {
        list_cmd.remove(0);
        exec_cmd_back(list_cmd,&filtered_env);
    }
    else {
        exec_cmd(list_cmd,&filtered_env);
    }
    // else {
    //     exec_cmd(list_cmd,&filtered_env);
    // }
}

fn space_split(user_input: &str) -> Vec<String> {
    let list_cmd_splited: Vec<_> = user_input.split_whitespace().map(|x| x.to_string()).collect();
    list_cmd_splited
}


#[warn(dead_code)]
fn exec_cmd(list_cmd : Vec<String> , env : &HashMap<String, String>) -> i32 {
    let  cmd = Command::new(&list_cmd[0])
        .args(&list_cmd[1..])
        .env_clear()
        .envs(env)
        .status()
        .expect("erreur");
    
    println!("process exited with: {}", cmd);
    1
}

//cette function exuecute des cmd en back end 
fn exec_cmd_back(list_cmd : Vec<String> , env : &HashMap<String, String>)   {
    let mut cmd = Command::new(&list_cmd[0])
        .args(&list_cmd[1..])
        // .status()
        .env_clear()
        .envs(env)
        .spawn()
        .expect("erreur");

    match cmd.try_wait() {
            Ok(Some(status)) => println!("exited with: {}", status),
            Ok(None) => {
                println!("status not ready yet, let's really wait");
                let res = cmd.wait();
                println!("result: {:?}", res);
            }
            Err(e) => println!("error attempting to wait: {}", e),
        }

    // println!("process exited with: {}", cmd);
    println!("pid: {}",cmd.id());
}

// cette function chnage de ripo 
fn cd(list_cmd : Vec<String>) {
    if list_cmd.is_empty() {
        panic!("pas darguùment")
    }
    let path = Path::new(&list_cmd[1]);
    set_current_dir(&path).unwrap();
    
}


// ----------------question2-----------------
fn main() -> std::io::Result<()> {
    loop {
        print!("user@machine$>");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();

        let mut user_input = String::with_capacity(256);

        // On prend une référence mutable
        stdin.read_line(&mut user_input)?;
        
        cmd_prossing(user_input);

        // `?` sert à « propager l'erreur » dans la fonction appellante
        // c'est mieux que de crash avec un unwrap ou expect ;)
        
    }
}