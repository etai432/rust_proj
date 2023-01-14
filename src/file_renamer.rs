use std::{io, fs, env, file};

pub fn run() {
    let abs_path = env::current_dir().unwrap();
    let mut root = String::from(abs_path.to_str().unwrap().split(r"\").collect::<Vec<&str>>()[0]);
    root.push_str(r"\");
    let mut current = String::from(root.clone());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to get input");
    input = String::from(input.trim());
    while input.ne("done") {
        let in_vec: Vec<&str> = input.split(" ").collect();
        if in_vec.len() > 0 {
            match in_vec[0] {
                "cmd" => println!("commands: ls, cd, root, env, this, current, back, cdp, cdr, rename"),
                "ls" => if in_vec.len() > 1{for file in fs::read_dir(&current).unwrap(){if String::from(file.as_ref().unwrap().path().to_str().unwrap()).contains(in_vec[1]){println!("{}", file.unwrap().path().display());}}}else{for file in fs::read_dir(&current).unwrap(){println!("{}", file.unwrap().path().display());}},
                "cd" => if in_vec.len() > 1{for file in fs::read_dir(&current).unwrap(){if file.as_ref().unwrap().path().to_str().unwrap().eq(&(current.clone() + in_vec[1..in_vec.len()].join(" ").as_str())){current+=in_vec[1..in_vec.len()].join(" ").as_str();current+=r"\";break;}}}else{println!("not given an argument")},
                "root" => current = String::from(root.clone()),
                "env" => current = String::from(abs_path.to_str().unwrap()) + r"\",
                "this" => current = String::from(abs_path.to_str().unwrap()) + r"\" + file!() + r"\",
                "current" => println!("{}", current),
                "back" => current = current.split(r"\").collect::<Vec<&str>>().split_last().unwrap().1.split_last().unwrap().1.join(r"\") + r"\",
                "cdp" => if in_vec.len() > 1 && fs::metadata(String::from(in_vec[1])).is_ok(){current=String::from(in_vec[1])+r"\"}else{println!("couldnt find path")}
                "cdr" => if in_vec.len() > 1 && fs::metadata(String::from(abs_path.to_str().unwrap())+r"\"+in_vec[1]).is_ok(){current=String::from(abs_path.to_str().unwrap())+r"\"+in_vec[1]+r"\"}else{println!("not given a path")},
                "rename" => if in_vec.len() > 1{fs::rename(&current[..current.len() - 1], current.split(r"\").collect::<Vec<&str>>().split_last().unwrap().1.split_last().unwrap().1.join(r"\") + r"\" + in_vec[1] + r"." + current.split(r".").collect::<Vec<&str>>()[1]).unwrap()},
                //add: create with text (touch), possibly add editing and deleting
                _ => println!("unknown command"),
            }
        }
        input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to get input");
        input = String::from(input.trim());
    }
}