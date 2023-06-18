use clap::Parser;
use std::fs::OpenOptions;
use std::process;
use std::io::{SeekFrom, Seek, Write};
use chrono::{Local, Duration};


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    filename: String,

    #[arg(short, long, default_value_t = 10)]
    countdown: u8,
}


fn main() {
    let args = Args::parse();

    match start_countdown(&args.filename, args.countdown) {
        Ok(_) => {
            println!("Countdown complete!")
        },
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            process::exit(1);
        }
    }
}

fn start_countdown(filename: &str, countdown: u8) -> Result<(), Box<dyn std::error::Error>> {
    let finish_time = Local::now() + Duration::minutes(countdown as i64);

    let mut file = open_file(filename)?;

    loop {
        let now = Local::now();

        if now >= finish_time {
            break;
        }

        let time_left = finish_time - now;
        let time_left_string = format!("{:0>2}:{:0>2}", time_left.num_minutes(), time_left.num_seconds() % 60);

        replace_with_text(&mut file, &time_left_string)?;

        std::thread::sleep(std::time::Duration::from_millis(250));
    }
    Ok(())
}

fn replace_with_text(file: &mut std::fs::File, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(text.as_bytes())?;
    file.flush()?;
    Ok(())
}

fn open_file(filename: &str) -> Result<std::fs::File, Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .write(true)
        .open(filename)?;
    Ok(file)
}