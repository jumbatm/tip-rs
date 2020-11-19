use std::path::PathBuf;
use structopt::StructOpt;
use tip::tip_parser;

#[derive(StructOpt, Debug)]
#[structopt(name = "tip")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
    #[structopt(short, long)]
    dump_ast: bool,
}

fn main() {
    let opt = Opt::from_args();
    if opt.dump_ast {
        let src = opt
            .files
            .into_iter()
            .fold(String::new(), |mut acc, current| {
                let src = std::fs::read_to_string(current).unwrap();
                acc.push('\n');
                acc.push_str(&src);
                acc
            });
        dbg!(&src);
        let ast = tip_parser::parse(src);
        println!("{:#?}", ast);
    }
}
