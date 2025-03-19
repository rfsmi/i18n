use clap::Parser;
use paste;

macro_rules! decl_puzzles {
    [
        { $($mods:tt)* }
        { $($labels:tt)* }
        { $($arms:tt)* }
        $puzzle:tt, $($rest:tt)*
    ] => {
        paste::paste!{ decl_puzzles![
            {
                $($mods)*
                mod [<puzzle_ $puzzle>];
            }
            {
                [< Puzzle $puzzle >],
                $($labels)*
            }
            {
                Puzzle::[< Puzzle $puzzle >] => {
                    let input = include_str!(concat!("../inputs/", $puzzle, ".txt"));
                    (stringify!($puzzle), [< puzzle_ $puzzle >]::solve(input).to_string())
                },
                $($arms)*
            }
            $($rest)*
        ]; }
    };
    (
        { $($mods:tt)* }
        { $($labels:tt)* }
        { $lhs:path => $rhs:expr, $($rest_lhs:path => $rest_rhs:expr,)* }
    ) => (
        #[derive(clap::ValueEnum, Copy, Clone, Debug)]
        enum Puzzle { $($labels)* Latest }

        $($mods)*

        fn run(args: Args) {
            let start = std::time::Instant::now();
            let (puzzle, result) = match args.puzzle {
                $lhs => $rhs,
                Puzzle::Latest => $rhs,
                $($rest_lhs => $rest_rhs,)*
            };
            let duration = start.elapsed().as_secs_f32();
            println!("Computed result for puzzle {puzzle} in {duration:.3} seconds: {result}");
        }
    );
    [$($rest:tt)*] => {
        decl_puzzles![{} {} {} $($rest)*];
    };
}

decl_puzzles![1, 2,];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum, default_value_t=Puzzle::Latest)]
    puzzle: Puzzle,
}

fn main() {
    run(Args::parse());
}
