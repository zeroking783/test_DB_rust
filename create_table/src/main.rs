use std::process::{Command, Stdio};
use std::io::{self, BufRead};
use dotenv::dotenv;

fn main() {
    let pass_db = dotenv::var("PASS_DB").expect("Enter the environment variable PASS_DB");
    let db_name = dotenv::var("DB_NAME").expect("Enter the environment variable DB_NAME");
    let db_user = dotenv::var("DB_USER").expect("Enter the environment variable DB_USER");

    let com = Command::new("psql")
        .arg("-h")
        .arg("localhost")
        .arg("-U")
        .arg(db_user)
        .arg("-d")
        .arg(db_name)
        .output()
        .expect("Failed 1");


    let stdout = String::from_utf8(com.stdout).expect("Failed 2");
    let stderr = String::from_utf8(com.stderr).expect("Failed 3");
    println!("Stdout: {}", stdout);
    println!("Stderr: {}", stderr);
}