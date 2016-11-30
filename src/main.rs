use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

fn main() {
    let mut branches = HashMap::new();
    branches.insert("master".to_string(), vec![Rc::new("Starting Payload".to_string())]);
    println!("master -> 'Starting Payload'");

    print!("> ");
    std::io::stdout().flush();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let unwrapped_line = line.unwrap();
        let sp = unwrapped_line.split(" ");
        let words: Vec<&str> = sp.collect();
        if words.len() > 2
        {
            match (words[0].to_string() + words[1]).as_ref(){
                "newbranch" => {
                    let old_branch : Vec<&str> = words[3].split("~").collect();
                    let old_branch_name = old_branch[0];
                    let after_num = if old_branch.len() == 2 {Some(old_branch[1])} else {None};
                    let new_branch_vec = match after_num {
                        Some(z) => {
                            let mut v = Vec::new();
                            let mut counter : usize = 0;
                            for x in branches.get(old_branch_name).unwrap() {
                                if counter >= branches.get(old_branch_name).unwrap().len() - usize::from_str(z).unwrap() {
                                    break;
                                } else {
                                    v.push(x.clone());
                                    counter += 1;
                                }
                            }
                            v
                        }
                        None => {
                            let mut v = Vec::new();
                            for x in branches.get(old_branch_name).unwrap() {
                                v.push(x.clone());
                            }
                            v
                        }
                    };
                    let new_name = words[2].to_string();
                    branches.insert(new_name, new_branch_vec);
                    println!("{} -> {}", words[2].to_string(), branches.get(&words[2].to_string()).unwrap().last().unwrap());
                } 
                "newcommit" => {
                    let w1 = words[words.len()-1];
                    let mut w2 = words[2].to_string();
                    for n in 3..(words.len()-1) {
                        w2 += " ";
                        w2 += words[n];
                    }
                    let br = branches.get_mut(w1);
                    match br {
                        Some(v) => {
                            v.push(Rc::new(w2.to_string()));
                            println!("{} -> {}", w1, w2);
                        }
                        None => println!("Error")
                    }
                    
                } 
                "deletebranch" => {
                    let removed_val = branches.remove(words[2]);
                    match removed_val
                    {
                        Some(mut z) => {
                            z.reverse();
                            println!("{} deleted", words[2].to_string());
                            for x in z {
                                match Rc::try_unwrap(x) {
                                    Ok(y) => {println!("{} deleted", y);}
                                    Err(_) => { }
                                }
                            }
                        }
                        None => {println!("Error")}
                    };
                } 
                _ => println!("Error")
            }
        }
        print!("> ");
        std::io::stdout().flush();
    }
}