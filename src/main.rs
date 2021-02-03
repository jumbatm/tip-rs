use std::path::PathBuf;
use structopt::StructOpt;
use tip::tip_parser;
use tip::cfg::IntraprocCFGBuilder;
use petgraph::dot::{Dot, Config};

#[derive(StructOpt, Debug)]
#[structopt(name = "tip")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
    #[structopt(short, long)]
    /// Dump the AST.
    dump_ast: bool,
    /// Dump the CFG in .dot format.
    #[structopt(short = "c", long)]
    dump_cfg: bool,
    #[structopt(long)]
    verbose: bool,
}

fn main() {
    let opt = Opt::from_args();
    let src = opt
        .files
        .into_iter()
        .fold(String::new(), |mut acc, current| {
            let src = std::fs::read_to_string(current).unwrap();
            acc.push_str(&src);
            acc.push('\n');
            acc
        });
    if opt.verbose {
        println!("Src is");
        for (idx, line) in src.lines().enumerate() {
            println!("{}\t| {}", idx+1, line);
        }
    }
    let ast = tip_parser::parse(src).unwrap();
    if opt.dump_ast {
        println!("{:#?}", ast);
    }
    let cfgs = IntraprocCFGBuilder::from_program(ast).to_owned_cfg_vec();
    if opt.dump_cfg {
        for cfg in cfgs {
            println!("{:#?}", Dot::with_config(&cfg, &[Config::EdgeNoLabel]));
        }
    }
}
