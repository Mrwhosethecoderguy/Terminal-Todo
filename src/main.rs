use std::fs;
use std::io;

fn main() {
    let data = fs::read_to_string("./TODO").expect("Unable to read file");
    let data = data.split_terminator("\n").collect::<Vec<&str>>();
    let mut todo = data[0].split_terminator(",").collect::<Vec<&str>>();
    let mut done = data[1].split_terminator(",").collect::<Vec<&str>>();

    for i in 0..todo.len() {
        todo[i] = todo[i].trim();
    }

    for i in 0..done.len() {
        done[i] = done[i].trim();
    }


    loop {
        show(&todo,&done);
        let mut command: String = String::new();
        io::stdin().read_line(&mut command);
    }
    
    
}

fn show(todo: &Vec<&str>,done: &Vec<&str>) {
    for i in 0..todo.len() {
        println!("{} [  ]", todo[i]);
    }
    for i in 0..done.len() {
        println!("{} [ x ]", done[i]);
    }
}


