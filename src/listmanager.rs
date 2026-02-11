use std::io;
use std::io::Write;

pub  fn run(){
#[derive(Debug)]
enum Status {
    Pending,
    Completed,
}

#[derive(Debug)]
struct Task {
    title: String,
    status: Status,
}

impl Task {
    fn new(title: String) -> Task {
        Task {
            title,
            status: Status::Pending,
        }
    }

    fn complete(&mut self) {
        self.status = Status::Completed;
    }

    fn display(&self, index: usize) {
        let status_symbol = match self.status {
            Status::Pending => "[ ]",
            Status::Completed => "[x]",
        };
        println!("{} {} - {}", index, status_symbol, self.title);
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        println!("\n=== To-Do List Manager ===");
        println!("1. Add Task");
        println!("2. Complete Task");
        println!("3. Show Tasks");
        println!("4. Exit");
        print!("Enter choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                print!("Enter task title: ");
                io::stdout().flush().unwrap();

                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read input");

                let task = Task::new(title.trim().to_string());
                tasks.push(task);
                println!("Task added!");
            }

            "2" => {
                print!("Enter task number to complete: ");
                io::stdout().flush().unwrap();

                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read input");

                match index.trim().parse::<usize>() {
                    Ok(i) if i > 0 && i <= tasks.len() => {
                        tasks[i - 1].complete();
                        println!("Task marked as completed!");
                    }
                    _ => println!("Invalid task number."),
                }
            }

            "3" => {
                if tasks.is_empty() {
                    println!("No tasks available.");
                } else {
                    for (i, task) in tasks.iter().enumerate() {
                        task.display(i + 1);
                    }
                }
            }

            "4" => {
                println!("Goodbye!");
                break;
            }

            _ => println!("Invalid choice, try again."),
        }
    }
}
}
