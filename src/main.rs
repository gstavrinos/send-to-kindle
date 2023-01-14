use clap::Parser;

#[tokio::main]
async fn main() -> thirtyfour::prelude::WebDriverResult<()> {
    let args = send_to_kindle::Args::parse();
    println!("{:?}",args);
    let mut f = String::from(args.directory.clone());
    if args.file.is_empty() && args.directory.is_empty(){
        eprintln!("Please specify a file or directory! (--file / --directory)");
        ()
    }
    else if args.directory.is_empty() && !args.file.is_empty() {
        f = String::from(args.file);
    }
    send_to_kindle::send_to_kindle(args.username.as_str(), args.password.as_str(), f.as_str(), args.extension.as_str(), args.file_timeout, args.amazon_url.as_str(), args.geckodriver_daemon, args.debugging_mode).await
}
