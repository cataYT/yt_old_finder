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

fn parse_range_or_single(range_or_single: String, max: usize) -> Vec<usize>
{
	if range_or_single.contains('-')
	{
		let splitted: Vec<&str> = range_or_single.split('-').collect();
		if splitted.len() != 2
		{
			eprintln!("Please enter a valid range like '3-5' or a single number.");
			return Vec::new();
		}

		let start: &str = splitted[0].trim();
		let end: &str = splitted[1].trim();

		let parsed_start: u32 = match start.parse()
		{
			Ok(num) => num,
			Err(_) => {
				eprintln!("Invalid start number");
				return Vec::new();
			}
		};

		let parsed_end: u32 = match end.parse()
		{
			Ok(num) => num,
			Err(_) => {
				eprintln!("Invalid end number");
				return Vec::new();
			}
		};

		if parsed_start < 1 || parsed_end > max as u32 || parsed_start > parsed_end
		{
			eprintln!("Range out of bounds or invalid");
			return Vec::new();
		}

		(parsed_start..=parsed_end)
			.map(|x| x as usize)
			.collect()
	}
	else
	{
		// single number case
		let parsed: u32 = match range_or_single.parse()
		{
			Ok(num) => num,
			Err(_) => {
				eprintln!("Invalid number");
				return Vec::new();
			}
		};

		if parsed < 1 || parsed > max as u32
		{
			eprintln!("Number out of bounds");
			return Vec::new();
		}

		vec![parsed as usize]
	};
	Vec::new()
}

fn save_videos(finder: &mut Finder)
{
	if finder.get_videos().len() == 0 { return; }
	let mut save_videos_input: String = String::new();
	get_input("Do you want to save any video? (yes/no): ", &mut save_videos_input);
	match save_videos_input.trim().to_ascii_lowercase().as_str()
	{
		"y" | "yes" => {
			let mut number_range_save = String::new();
			let max = finder.get_videos().len();

			let msg = format!("Enter the video you want to save (1-{} in range): ", max);
			get_input(&msg, &mut number_range_save);

			number_range_save = number_range_save.trim().to_string();
			let result: Vec<usize> = parse_range_or_single(number_range_save, max);
			if result.len() == 0 { return; }
			if let Err(e) = finder.save_videos(result)
			{
				eprintln!("Failed to save videos: {}", e);
			}
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
	}
}