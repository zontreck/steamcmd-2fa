use clap::Parser;
use simple_steam_totp::generate;

fn find_default_steamcmd() -> &'static str {
    if cfg!(target_os = "windows") {
        "C:\\steamcmd\\steamcmd.exe"
    } else {
        if std::path::Path::new("/home/steam/steamcmd/steamcmd.sh").exists() {
            "/home/steam/steamcmd/steamcmd.sh"
        } else {
            "/home/steam/steamcmd"
        }
    }
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    // Path to steamcmd executable
    #[clap(long, default_value = find_default_steamcmd())]
    path: String,

    // Steam username
    #[clap(short, default_value = "")]
    username: String,

    // Steam password
    #[clap(short, default_value = "")]
    password: String,

    // Steam 2FA shared secret
    #[clap(short, long)]
    secret: String,

    // For raw output only
    #[clap(long)]
    raw: bool, // Changed from Bool to bool

    // Steamcmd args
    #[clap(short, default_value = "+quit")]
    args: String,
}

fn main() {
    let args = Args::parse();


    let totp = match generate(&args.secret) {
        Ok(code) => code,
        Err(e) => {
            println!("Failed to generate Steam TOTP code: {}", e);
            std::process::exit(1);
        }
    };

    if args.raw {
        println!("{}", totp); // Use println! and correct the variable name
        std::process::exit(0); // Use std::process::exit(0) to return a proper exit code
    }else {
        if args.password == "" {
            println!("Password is required");
            std::process::exit(1);
        }

        if args.username == "" {
            println!("Username is required");
            std::process::exit(1);
        }
    }

    if !std::path::Path::new(&args.path).exists() {
        println!("Steamcmd executable not found at {}. Please specify with --path", args.path);
        std::process::exit(1);
    }


    let cmd_arg = format!("+login {} {} {} {}", &args.username, &args.password, &totp, &args.args);

    let mut cmd = std::process::Command::new(&args.path);
    cmd.arg(&cmd_arg);

    println!("{} {:?}\n", &args.path, &cmd_arg.replace(&args.username, "****").replace(&args.password, "****").replace(&totp, "****"));

    std::process::exit(cmd.status().unwrap().code().unwrap());
}
