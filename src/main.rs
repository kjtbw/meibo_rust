use std::io;
use std::process;
use chrono::{Utc, Local, DateTime, Date};

#[derive(Debug)]
struct ProfileCollection {
    v: Vec<Profile>,
}

impl ProfileCollection{
    fn push(&mut self, p: Profile){
        self.v.push(p);
    }
}

#[derive(Debug)]
struct Profile {
    id: i32,
    name: String,
    year: Date<Utc>,
    place: String,
    note: String,
}



#[derive(Debug)]
enum Message {
    Quit,
    Column,
    Print(i32),
    Read(String),
    Write(String),
    Find(String),
    Sort(i32),
    NotFound,
}

impl Message {
    fn call(&self, pc:ProfileCollection) -> i32 {
        match self{
            Message::Quit => self.quit(),
            Message::Column => self.column(pc),
            Message::Print(n) => self.print(pc),
            Message::Read(s) => self.read(pc),
            Message::Write(s) => self.write(pc),
            Message::Find(s) => self.find(pc),
            Message::Sort(n) => self.sort(pc),
            Message::NotFound => self.quit(),
        }
    }//end method call

    fn quit(&self) -> i32{
        println!("Your Command is {:?}", self);
        process::exit(1)
    }//end method quit

    fn column(&self, pc:ProfileCollection) -> i32{
        println!("Your Command is {:?}", self);
        1
    }//end method column

    fn print(&self, pc:ProfileCollection) -> i32{
        println!("Your Command is {:?}", self);
        2
    }//end method print

    fn read(&self, pc:ProfileCollection) -> i32{
        println!("Your Command is {:?}", self);
        3
    }//end method read

    fn write(&self, pc:ProfileCollection) -> i32{
        println!("Your Command is {:?}", self);
        4
    }//end method write

    fn find(&self, pc:ProfileCollection) -> i32{
        println!("Your Command is {:?}", self);
        5
    }//end method find

    fn sort(&self, pc:ProfileCollection) -> i32{
        println!("Your Command is {:?}", self);
        6
    }//end method sort

}//end impl Message

fn evaluate_input(s: &String, mut pc: ProfileCollection) {
    if s.starts_with("%") {
        let m = gen_message(s);
        m.call(pc);
    } else {
        // csv形式の保存と，エラー処理
        let data: Vec<&str> = s.split(",").collect();
        let p = Profile {
            id: data.get(0).unwrap().parse().expect("Failed to convert &str into i32"),
            name: data.get(1).unwrap().to_string(),
            year: Utc::today(),
            place: data.get(3).unwrap().to_string(),
            note: data.get(4).unwrap().to_string(),
        };
        pc.push(p);
        println!("Your input: {:?} ", pc);
    }
}

fn gen_message(s: &String) -> Message {
    // %のあとの添字に応じて適切なMessage型を生成する処理
    match s.chars().nth(1).unwrap(){
        'Q' => Message::Quit,
        'C' => Message::Column,
        'P' => {let n: i32 = s.split(" ").nth(1).unwrap().parse().expect("Failed to convert &str into i32");
                Message::Print(n)},
        'R' => Message::Read(s.split(" ").nth(1).unwrap().to_string()),
        'W' => Message::Write(s.split(" ").nth(1).unwrap().to_string()),
        'F' => Message::Find(s.split(" ").nth(1).unwrap().to_string()),
        'S' => {let n: i32 = s.split(" ").nth(1).unwrap().parse().expect("Failed to convert &str into i32");
                Message::Sort(n)},
        _ => Message::NotFound,
    }
}

fn main() {
    // ほんとはライフタイム渡したい
    // let mut pc = ProfileCollection {
    //     v: Vec::new(),
    // };

    loop{
        let mut pc = ProfileCollection {
            v: Vec::new(),
        };

        let mut guess = String::new();
        println!("Please input some message!");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("Your input: {} ", guess);
        guess.pop();// 末尾の改行削除
        evaluate_input(&guess, pc);
    }
}//end main
