use postgres::{NoTls, Client, Error};
use csv::{ReaderBuilder};

fn main() {
    let adress = dotenv::var("ADRESS").expect("Enter the environment variable ADRESS");
    let port = dotenv::var("PORT").expect("Enter the environment variable PORT");
    let pass_db = dotenv::var("PASS_DB").expect("Enter the environment variable PASS_DB");
    let db_name = dotenv::var("DB_NAME").expect("Enter the environment variable DB_NAME");
    let db_user = dotenv::var("DB_USER").expect("Enter the environment variable DB_USER");
    let replica_number = dotenv::var("REPLICA_NUMBER").expect("REPLICA_NUMBER is not found");
    let tb_name = dotenv::var("TB_NAME").expect("TB_NAME is not found");
    let client: Client;

    let mut client = connect_to_database(&adress, &port, &pass_db, &db_name, &db_user)
        .unwrap_or_else(|e| {
            eprintln!("Error: {}", e);
            panic!("Failed to connect to database"); // Или другое действие в случае ошибки
        });

    let file_name = String::from(format!("/var/lib/DB_test/{}.csv", replica_number));

    let replica_number = replica_number.parse::<i32>().expect("Failed to cast replica_number to correct type");

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_name)
        .expect("Failed to open csv file");

    for result in rdr.records() {
        let record = result.expect("Error reading csv file");
        let word = &record[0][..];
        let number = &record[1].parse::<i32>().expect("Failed to cast number to correct type");

        let query = format!("INSERT INTO {} (replica, word, number) VALUES ($1, $2, $3)", tb_name);
        let rows_updated = client.execute(
            &query,
            &[&replica_number, &word, &number],
        ).expect("Failed to insert row into table");
        println!("word: {}", &record[0]);
    }


    break_connection(client);
}

fn connect_to_database(adress: &String, port: &String, pass_db: &String, db_name: &String, db_user: &String) -> Result<Client, Error> {
    let mut client = Client::connect(format!("postgresql://{}:{}@{}:{}/{}", &db_user, &pass_db, &adress, &port, &db_name).as_str(), NoTls)?;
    Ok(client)
}

fn break_connection(client: Client) {}

