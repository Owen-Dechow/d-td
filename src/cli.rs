pub mod actions {
    use clap::{Args, Parser, Subcommand};

    #[derive(Parser)]
    #[clap(author = "Owen G. Dechow", version = "1.0", about = "Todo App")]
    pub struct Arguments {
        #[clap(subcommand)]
        pub action: Action,
    }

    #[derive(Subcommand)]
    pub enum Action {
        #[clap(about = "Add a new item")]
        Add(AddArgs),

        #[clap(about = "Delete an item")]
        Delete(DeleteArgs),

        #[clap(about = "List all items")]
        List(ListArgs),

        #[clap(about = "Toggle the state of an item")]
        Toggle(ToggleArgs),

        #[clap(about = "Move item from one position to another")]
        Move(MoveArgs),

        #[clap(about = "Alphabatize list")]
        Order(OrderArgs),

        #[clap(about = "Initialize new list at current position")]
        Init(InitArgs),

        #[clap(about = "Destroy db")]
        Destroy(DestroyArgs),
    }

    #[derive(Args)]
    pub struct AddArgs {
        pub item: String,
    }

    #[derive(Args)]
    pub struct DeleteArgs {
        pub index: usize,
    }

    #[derive(Args)]
    pub struct ToggleArgs {
        pub index: usize,
    }

    #[derive(Args)]
    pub struct ListArgs {}

    #[derive(Args)]
    pub struct MoveArgs {
        pub index: usize,
        pub to: usize,
    }

    #[derive(Args)]
    pub struct OrderArgs {
        #[clap(short, long, default_value_t = false)]
        pub reverse: bool,
    }

    #[derive(Args)]
    pub struct InitArgs {}

    #[derive(Args)]
    pub struct DestroyArgs {}
}
