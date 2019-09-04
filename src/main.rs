use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "git-timeshift", rename_all = "kebab-case")]
/// the stupid content tracker
pub enum Opt {
    Rec {
    },
    Restore {
        timeshift: String,
    },
    #[structopt(name = "ls")]
    List {
    }
}

fn main() {
    let matches = Opt::from_args();

    println!("{:?}", matches);
}
