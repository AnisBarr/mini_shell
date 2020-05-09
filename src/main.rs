use std::io::{self,Write};
use std::process::{Command};
use std::path::Path;
use std::env;
use std::env::set_current_dir;
use std::collections::HashMap;




#[warn(unused_variables)]
fn cmd_prossing(cmd : String){
    let mut list_cmd = space_split(cmd.as_str());

    let filtered_env : HashMap<String, String> =  env::vars().filter(|&(ref k, _)| k == "TERM" || k == "TZ" || k == "LANG" || k == "PATH").collect();

    if cmd.as_str().contains("|"){

        let  list_cmd_splited :  Vec<String> = cmd.split("|").map(|x| x.to_string()).collect();
        let  list_cmd_1 = space_split(list_cmd_splited[0].as_str());
        let  list_cmd_2 = space_split(list_cmd_splited[1].as_str());

        exec_pipe(list_cmd_1,list_cmd_2, &filtered_env);
    }

    if list_cmd[0].as_str() == "cd" {
        cd(list_cmd);
    }
    else if list_cmd[0].as_str() == "back" {
        list_cmd.remove(0);
        exec_cmd_back(list_cmd,&filtered_env);
    }
    else {
        exec_cmd(list_cmd,&filtered_env);
    }

}


fn exec_pipe (_list_cmd_1 : Vec<String> , _list_cmd_2 : Vec<String> , _env : &HashMap<String, String>)  {
    
    // je sais pas comment le fair mais voila l'idée je prends les 2 cmd et l'env et c'est la que je fait de la magie 
}



fn space_split(user_input: &str) -> Vec<String> {
    let list_cmd_splited: Vec<_> = user_input.split_whitespace().map(|x| x.to_string()).collect();
    list_cmd_splited
}



fn exec_cmd(list_cmd : Vec<String> , env : &HashMap<String, String>) -> std::process::ExitStatus {
    let  cmd = Command::new(&list_cmd[0])
        .args(&list_cmd[1..])
        .env_clear()
        .envs(env)
        .status()
        .expect("erreur");
    
    println!("process exited with: {}", cmd);
    cmd
}

//cette function exuecute des cmd en back end 
fn exec_cmd_back(list_cmd : Vec<String> , env : &HashMap<String, String>)   -> std::process::Child {
    let  cmd = Command::new(&list_cmd[0])
        .args(&list_cmd[1..])
        .env_clear()
        .envs(env)
        .spawn()
        .expect("erreur");

    // match cmd.try_wait() {
    //         Ok(Some(status)) => println!("exited with: {}", status),
    //         Ok(None) => {
    //             println!("status not ready yet, let's really wait");
    //             let res = cmd.wait();
    //             println!("result: {:?}", res);
    //         }
    //         Err(e) => println!("error attempting to wait: {}", e),
    //     }

    println!("pid: {}",cmd.id());
    cmd
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