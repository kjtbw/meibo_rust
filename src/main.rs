use std::io;
use std::process;
//use chrono::{Utc, Local, DateTime, Date};
use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

///////////////////////////////////////////////
// define Profile
///////////////////////////////////////////////
#[derive(Debug)]
struct Profile {
    id: i32,
    name: String,
    year: String,
    place: String,
    note: String,
}

impl Profile {
    fn print_self(&self){
        println!("Id:{}\nName:{}\nYear:{}\nPlace:{}\nNote:{}\n", self.id, self.name, self.year, self.place, self.note);
    }

    fn new(data:Vec<&str>) -> Profile{
        let p = Profile {
            id: data.get(0).unwrap().parse().expect("Failed to convert &str into i32"),
            name: data.get(1).unwrap().to_string(),
            year: data.get(2).unwrap().to_string(),
            place: data.get(3).unwrap().to_string(),
            note: data.get(4).unwrap().to_string(),
        };
        p
    }

    fn to_csv(&self) -> String{
        let csv = format!("{},{},{},{},{}\n", self.id, self.name, self.year, self.place, self.note);
        csv
    }

    fn contains(&self, w:&str) -> bool{
        self.id.to_string().contains(w) || self.name.contains(w) || self.year.contains(w) || self.place.contains(w) || self.note.contains(w)
    }

}//end impl Profile

///////////////////////////////////////////////
// define ProfileCollection
///////////////////////////////////////////////
pub trait ProfileCollection {
    fn print_list(&self, n:u32);
}

impl ProfileCollection for Vec<Profile> {
    fn print_list(&self, n:u32){
        for i in 0..n {
            let i = i as usize;
            self[i].print_self();
        }
    }
}

///////////////////////////////////////////////
// define Message
///////////////////////////////////////////////
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
    fn call(&self, v:&mut Vec<Profile>){
        match &self{
            Message::Quit => self.quit(),
            Message::Column => self.column(v),
            Message::Print(n) => self.print(*n, v),
            Message::Read(s) => self.read(s, v),
            Message::Write(s) => self.write(&s, v),
            Message::Find(s) => self.find(&s, v),
            Message::Sort(n) => self.sort(*n, v),
            Message::NotFound => self.quit(),
        }
    }//end method call

    fn quit(&self){
        process::exit(1);
    }//end method quit

    fn column(&self, v:&mut Vec<Profile>){
        println!("{} Profile(s)", v.len());
    }//end method column

    fn print(&self, n:i32, v:&mut Vec<Profile>){
        match n.cmp(&0) {
            Ordering::Greater => {
                let n = n as u32;
                v.print_list(n);
            },
            Ordering::Equal => {
                let n = v.len() as u32;
                v.print_list(n);
            },
            Ordering::Less => {
                v.reverse();
                let n = n.abs() as u32;
                v.print_list(n);
                v.reverse();
            }
        }
    }//end method print

    fn read(&self, s:&String, v:&mut Vec<Profile>){
        let mut file = File::open(s).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        let list = contents.split("\n");
        for p in list {
            if p.contains(",") {
                let data: Vec<&str> = p.split(",").collect();
                let p = Profile::new(data);
                v.push(p);
            }
        }
        println!("Command R is finished");
    }//end method read

    fn write(&self, s:&String, v:&mut Vec<Profile>){
        let mut buffer = File::create(s).unwrap();
        let mut s = String::new();
        for p in v{
            let csv = p.to_csv();
            s.push_str(&csv);
        }
        write!(buffer, "{}", s);
        println!("Command W is finished");
    }//end method write

    fn find(&self, s:&String, v:&mut Vec<Profile>){
        for p in v{
            if p.contains(s) {
                p.print_self();
            }
        }
        println!("Command F is finished");
    }//end method find

    fn sort(&self, n:i32, v:&mut Vec<Profile>){
        match n {
            1 => v.sort_by(|a,b| a.id.cmp(&b.id)),
            2 => v.sort_by(|a,b| a.name.cmp(&b.name)),
            3 => v.sort_by(|a,b| a.year.cmp(&b.year)),
            4 => v.sort_by(|a,b| a.place.cmp(&b.place)),
            5 => v.sort_by(|a,b| a.note.cmp(&b.note)),
            _ => println!("ERROR:Invarid Format")
        }
        println!("Command S is finished");
    }//end method sort
}//end impl Message

///////////////////////////////////////////////
// some functions for parse input
///////////////////////////////////////////////
fn evaluate_input(s: &String, v: &mut Vec<Profile>) {
    if s.starts_with("%") {
        let m = gen_message(s);
        m.call(v);
    } else {
        // csv形式の保存と，エラー処理
        if !s.contains(",") {
            println!("ERROR:Invarid Format");
            return;
        }
        let data: Vec<&str> = s.split(",").collect();
        let p = Profile::new(data);
        v.push(p);
        println!("Added Profile");
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

///////////////////////////////////////////////
// main
///////////////////////////////////////////////
fn main() {
    let mut v: Vec<Profile> = Vec::new();
    loop{
        let mut guess = String::new();
        println!("Please input some message!");
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess.pop();// 末尾の改行削除
        evaluate_input(&guess, &mut v);
    }
}//end main
