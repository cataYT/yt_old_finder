mod finder;
mod detail;
mod helper;

use finder::Finder;
use std::{env, process::exit};

#[tokio::main]
async fn main()
{
	let mut finder: Finder = Finder::new();
	if env::args().len() != 1 
	{
		helper::parse_args(&mut finder, env::args().skip(1).collect()).await;
		drop(finder);
		exit(0);
	}
	let mut video_format_input: String = String::new();
	helper::get_input("Enter the video format: ", &mut video_format_input);
	video_format_input = video_format_input
		.trim()
		.to_ascii_lowercase();
	for format in &finder.get_formats()
	{
		if !format.contains(video_format_input.as_str()) { return; }
		let mut number_of_videos_input: String = String::new();
		helper::get_input("Enter your amount of videos to open: ", &mut number_of_videos_input);
		match number_of_videos_input.trim().parse::<u32>()
		{
			Ok(_) => {
				helper::get_results(&mut finder, number_of_videos_input.as_str(), video_format_input.as_str()).await;
				helper::save_videos(&mut finder);
			},
			Err(err) => eprintln!("Failed to parse number of videos: {}", err) 
		}
	}
}