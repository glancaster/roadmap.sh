
use clap::{Args, Parser, Subcommand};

use rusqlite::{Connection, Result};

use env_logger::{self, Env};
use log::{info, debug, warn};

use chrono::prelude::*;

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
    Update(UpdateArgs),
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
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=12))]
    month: Option<u8>,
}

#[derive(Args)]
struct UpdateArgs {
    #[arg(long)]
    id: u32,
    #[arg(short, long)]
    description: Option<String>,
    #[arg(short, long)]
    amount: Option<u32>,
}

#[derive(Args)]
struct IDArgs {
    #[arg(long)]
    id: u32,
}

fn main() -> Result<()> {

    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();
    
    let cli = Cli::parse();

    let conn = Connection::open("tasks.db")?;

    conn.execute(
        "CREATE TABLE if not exists tasks (
            id              INTEGER PRIMARY KEY,
            date            TEXT NOT NULL,
            description     TEXT NOT NULL,
            amount          INT NOT NULL
        )",
        (), 
    )?;

    match &cli.command {
        Commands::Add(args) => {
            info!("'add' was requested, description is: {:?} and amount is {:?}", args.description, args.amount);
            
            let utc: NaiveDate = Utc::now().date_naive();

            conn.execute(
                "INSERT INTO tasks (date, description, amount) VALUES (?1, ?2, ?3)",
                (&utc.to_string(), &args.description, &args.amount),
            )?;
            let id = conn.last_insert_rowid();
            info!("Expense added successfully (ID: {0})", id);
            
        }
        Commands::List => {
            info!("'list' was requested");
            let mut stmt = conn.prepare("SELECT id, date, description, amount FROM tasks")?;
            let tasks_iter = stmt.query_map([], |row| {
                Ok((
                    row.get::<_,u32>(0)?,
                    row.get::<_,String>(1)?,
                    row.get::<_,String>(2)?,
                    row.get::<_,u32>(3)?,
                ))
            })?;

            println!("{0:5}{1:15}{2:40}{3:10}", "ID", "Date", "Description", "Amount");
            for task in tasks_iter {
                if let Ok(t) = task {
                    println!("{0:^5}{1:15}{2:40}{3:^10}", t.0, t.1, t.2, t.3);
                }
            }
        }
        Commands::Update(args) => {
            info!("'update' was requested, id is: {:?}", args.id);
            match (args.description.clone(), args.amount) {
                (Some(desc), Some(amt)) => {
                    conn.execute(
                        "UPDATE tasks SET description = ?2, amount = ?3 WHERE id = ?1",
                        [&args.id.to_string(), &desc, &amt.to_string()]
                    )?;
                }
                (Some(desc), None) => {
                    conn.execute(
                        "UPDATE tasks SET description = ?2 WHERE id = ?1",
                        [&args.id.to_string(), &desc]
                    )?;
                }
                (None, Some(amt)) => {
                    conn.execute(
                        "UPDATE tasks SET amount = ?2 WHERE id = ?1",
                        [&args.id.to_string(), &amt.to_string()]
                    )?;
                }
                _ => {
                    warn!("No Update parameters passed");
                }
            }
        }
        Commands::Delete(args) => {
            info!("'delete' was requested, id is: {:?}", &args.id);
            conn.execute(
                "DELETE FROM tasks WHERE id = ?1",
                [args.id]
            )?;
            info!("Expense deleted successfully");
        }
        Commands::Summary(args) => {
            info!("'summary' was requested, month is: {:?}", args.month);
            if let Some(month) = args.month {
                if matches!(month, 1..=12) {
                    let sum: u32 = conn.query_row("SELECT COALESCE(sum(amount), 0) FROM tasks WHERE strftime('%m', date) = ?1", [format!("{0:02}", month)], |row| {
                        row.get(0)
                    })?;
                    let month_str = match month {
                        1 => "January",
                        2 => "February",
                        3 => "March",
                        4 => "April",
                        5 => "May",
                        6 => "June",
                        7 => "July",
                        8 => "August",
                        9 => "September",
                        10 => "October",
                        11 => "November",
                        12 => "December",
                        _ => "Wrong Planet!",
                    };
                    println!("Total Expenses for {month_str}: ${sum}");
                    
                } else {
                    warn!("Argument passed for Month is not within 1 to 12");
                }
            } else {
                let sum: u32 = conn.query_row("SELECT COALESCE(sum(amount), 0) FROM tasks", [], |row| {
                    row.get(0)
                })?;

                println!("Total Expenses: ${sum}");
            }
        }
    }
    
    debug!("Done processing command");


    Ok(())
}

