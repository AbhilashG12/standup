use clap::Parser;

#[derive(Parser,Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    
    #[arg(short,long)]
    name : String,


    #[arg(short,long,default_value_t=1)]
    count : u8,

    #[arg(short,long)]
    shout : bool,

}


fn main(){
    let args = Args::parse();

    for _ in 0..args.count{
        let mut msg = format!("Hello {} " ,args.name);

        if args.shout{
            msg = msg.to_uppercase();
        }

        println!("{}",msg);
    }
}
