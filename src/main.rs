
use std::fs;
use std::io;

fn main() {
    let data = fs::read_to_string("./TODO.check").expect("Unable to read file");
    let data = data.split_terminator("\n").collect::<Vec<&str>>();
    let mut todo_str = data[0].split_terminator(",").collect::<Vec<&str>>();
    let mut done_str = data[1].split_terminator(",").collect::<Vec<&str>>();
    let mut todo: Vec<Item> = Vec::new();
    let mut done: Vec<Item> = Vec::new();

    for i in 0..todo_str.len() {
        todo_str[i] = todo_str[i].trim();
        todo.push(Item { content: todo_str[i].to_string(), done: false });
    }

    for i in 0..done_str.len() {
        done_str[i] = done_str[i].trim();
        done.push(Item {content: done_str[i].to_string(), done: true}); 
    }


    loop {
        show(&todo,&done);
        let mut command: String = String::new();

        io::stdin().read_line(&mut command).expect("Could not read line");
        command = command.trim().to_string();
        let mut content: String = String::new();
        if command == "add" {
            io::stdin().read_line(&mut content).expect("Could not read line");
            let n_item = Item{content: content.trim().to_string(), done: false};

            todo.push(n_item);
        }
        else if command == "tick" {
            io::stdin().read_line(&mut content).expect("Could not read line");
            let idx: u32 = content.trim().parse().expect("Enter a number");
            if idx > todo.len().try_into().unwrap() {
                println!("Please enter a valid number");
                continue;
            }
            let idx: usize = (idx-1).try_into().unwrap();
            done.push(Item {content: todo[idx].content.to_string(), done: true});
            todo.remove(idx);
        }
        else if command == "untick" {
            io::stdin().read_line(&mut content).expect("Could not read line");
            let idx: u32 = content.trim().parse().expect("Enter a number");
            if idx > todo.len().try_into().unwrap() {
                println!("Please enter a valid number");
                continue;
            }
            let idx: usize = (idx-1).try_into().unwrap();
            todo.push(Item {content: done[idx].content.to_string(), done: false});
            done.remove(idx);

        }
    }


    
    
}

fn show(todo: &Vec<Item>,done: &Vec<Item>) {
    println!("\n\n\t\t\tTODO: ");

    for (i,idx) in todo.iter().enumerate() {
        idx.disp(&i);
    }
    println!("\n\n\t\t\tDONE: ");
    for (i,idx) in done.iter().enumerate() {
        idx.disp(&i);
    }
}

struct Item {
    content: String,
    done: bool
}

impl Item {
    fn disp(&self,idx: &usize) {
        let x = if self.done {"x"} else {" "};
        println!("\t\t\t{}. {} [ {} ]",idx+1,self.content,x)
    }

}