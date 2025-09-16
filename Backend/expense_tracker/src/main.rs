
use clap::{Args, Parser, Subcommand};

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

fn main() {
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

