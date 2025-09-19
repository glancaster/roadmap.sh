
use clap::{Args, Parser, Subcommand};
use rusqlite::{Connection, Result};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add an Expense with description and amount
    Add(AddArgs),
    /// View all Expenses
    List,
    /// Update an Expense from ID
    Update(IDArgs),
    /// Delete an Expense from ID
    Delete(IDArgs),
    /// Summary for all Expenses. Optional Filter for Month
    Summary(SummaryArgs),
}

#[derive(Args)]
struct AddArgs {
    #[arg(short, long)]
    description: String,
    #[arg(short, long)]
    amount: u32,
}

#[derive(Args)]
struct SummaryArgs {
    #[arg(short, long)]
    month: Option<u8>,
}

#[derive(Args)]
struct IDArgs {
    #[arg(long)]
    id: u32,
}

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => {
            println!("'add' was used, description is: {:?} and amount is {:?}", args.description, args.amount);
        }
        Commands::List => {
            println!("'list' was used");
        }
        Commands::Update(args) => {
            println!("'update' was used, id is: {:?}", args.id);
        }
        Commands::Delete(args) => {
            println!("'delete' was used, id is: {:?}", args.id);
        }
        Commands::Summary(args) => {
            println!("'summary' was used, month is: {:?}", args.month);
        }
    }

    let conn = Connection::open_in_memory()?;

    conn.execute(
        "CREATE TABLE person (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            data  BLOB
        )",
        (), // empty list of parameters.
    )?;
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }



    Ok(())
}

struct ExpenseTracker;

//enum Month;

impl ExpenseTracker {
    //fn add(description:&str, amount:i32) {}
    //fn update(id) {}
    //fn delete(id) {}
    //fn list() {}
    //fn summary(Option<Month>) {}
    //// Extra
    //fn filter_category() {}
    //fn set_and_check_budget() {}
    //fn export_csv() {}
}

