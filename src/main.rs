use clap::Parser;

#[derive(Parser)]
struct Arguments {
    #[arg(short , long)]
    info: String,
    #[arg(short , long)]
    codename: String,
}

fn main(){
    let args =  Arguments::parse();

    println!("This is first dimploma project test {} ",args.info );
    println!("codename of the project is diploma-core {} " , args.codename);
}
