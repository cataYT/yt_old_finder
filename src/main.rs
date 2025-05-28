mod finder;
mod detail;
use std::io::Write;
use finder::Finder;

fn get_input(message: &str, input: &mut String)
{
	print!("{}", message);
	std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(input).unwrap();
}

async fn get_results(finder: &mut Finder, number_of_videos_input: &str, video_format_input: &str)
{
	match finder.query_videos(&*number_of_videos_input, Some(&*video_format_input)).await
	{
		Ok(response) =>
			{
				let base_url: &str = "https://www.youtube.com/watch?v=";
				for item in response.items
				{
					let video_id: &str = item.id.video_id.as_str();
					println!("Title: {}", item.snippet.title);
					println!("Video ID: {}", video_id);
					println!("Channel: {}", item.snippet.channel_title);
					println!("Description: {}\n", item.snippet.description);

					let complete_url: String = base_url.to_string() + video_id;
					finder.add_videos(complete_url.clone());

					webbrowser::open(&complete_url).unwrap();
				}
			}
		Err(err) => {
			eprintln!("Error: {}", err);
		}
	}
}

fn save_videos(finder: &mut Finder)
{
	if finder.get_videos().len() == 0 { return; }
	let mut save_videos_input: String = String::new();
	get_input("Do you want to save any video? (yes/no): ", &mut save_videos_input);
	match save_videos_input.trim().to_ascii_lowercase().as_str()
	{
		"y" | "yes" => {
			let mut number_save: String = String::new();
			let msg: String = format!
			(
				"Enter the video you want to save (1-{}): ",
				finder.get_videos().len() + 1
			);
			get_input(&msg, &mut number_save);
			match number_save.trim().parse::<u32>()
			{
				Ok(n) => {
					finder.save_videos(n as usize).unwrap();
				}
				Err(_) => {}
			}
			drop(number_save);
		}
		_ => return
	}
}

#[tokio::main]
async fn main()
{
	let mut finder: Finder = Finder::new();
	let mut video_format_input: String = String::new();
	get_input("Enter the video format: ", &mut video_format_input);
	video_format_input = video_format_input
		.trim()
		.to_ascii_lowercase();
	for format in &finder.get_formats()
	{
		if !format.contains(video_format_input.as_str()) { return; }
		let mut number_of_videos_input: String = String::new();
		get_input("Enter your amount of videos to open: ", &mut number_of_videos_input);
		match number_of_videos_input.trim().parse::<u32>()
		{
			Ok(_) => {
				get_results(&mut finder, number_of_videos_input.as_str(), video_format_input.as_str()).await;
				save_videos(&mut finder);
			},
			Err(err) => eprintln!("Failed to parse number of videos: {}", err) 
		}
		drop(number_of_videos_input);
	}
	drop(video_format_input);
}