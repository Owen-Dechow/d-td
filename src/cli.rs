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

        #[clap(about = "Clear all items from list")]
        Clear(ClearArgs),

        #[clap(about = "Toggle the state of an item")]
        Toggle(ToggleArgs),

        #[clap(about = "List all items")]
        List(ListArgs),

        #[clap(about = "Move item from one position to another")]
        Move(MoveArgs),

        #[clap(about = "Shift an item from one position to another")]
        Shift(ShiftArgs),

        #[clap(about = "Order list alphabetically")]
        OrderAlpha(OrderAlphaArgs),

        #[clap(about = "Order list by date")]
        OrderDate(OrderDateArgs),

        #[clap(about = "Get the path of targeted db")]
        Declare(DeclareArgs),

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
        pub index: i8,
    }

    #[derive(Args)]
    pub struct ToggleArgs {
        pub index: i8,
    }

    #[derive(Args)]
    pub struct ListArgs {}

    #[derive(Args)]
    pub struct MoveArgs {
        pub index: i8,
        pub to: i8,
    }

    #[derive(Args)]
    pub struct OrderAlphaArgs {
        #[clap(
            short,
            long,
            default_value_t = false,
            help = "Set to reverse alphabetical order"
        )]
        pub reverse: bool,

        #[clap(
            short,
            long,
            default_value_t = false,
            help = "Make ordering case sensitive"
        )]
        pub strict: bool,
    }

    #[derive(Args)]
    pub struct OrderDateArgs {
        #[clap(
            short,
            long,
            default_value_t = false,
            help = "Set to reverse alphabetical order"
        )]
        pub reverse: bool,
    }

    #[derive(Args)]
    pub struct InitArgs {}

    #[derive(Args)]
    pub struct DestroyArgs {}

    #[derive(Args)]
    pub struct DeclareArgs {}

    #[derive(Args)]
    pub struct ShiftArgs {
        pub index: i8,
        pub shift: i8,
    }

    #[derive(Args)]
    pub struct ClearArgs {}
}

pub mod colors {
    // pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    // pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    // pub const MAGENTA: &str = "\x1b[35m";
    // pub const CYAN: &str = "\x1b[36m";
    // pub const WHITE: &str = "\x1b[37m";
    pub const RESET: &str = "\x1b[0m";
}
