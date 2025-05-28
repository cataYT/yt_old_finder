use reqwest::Error;
use reqwest::Client;
use rand::Rng;
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use serde::Deserialize;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Deserialize)]
struct SearchResponse
{
	items: Vec<VideoItem>,
}

#[derive(Debug, Deserialize)]
struct VideoItem
{
	id: VideoId,
	snippet: Snippet,
}

#[derive(Debug, Deserialize)]
struct VideoId
{
	#[serde(rename = "videoId")]
	video_id: String,
}

#[derive(Debug, Deserialize)]
struct Snippet
{
	title: String,
	description: String,
	#[serde(rename = "channelTitle")]
	channel_title: String,
}

fn get_input(message: &str, input: &mut String)
{
	print!("{}", message);
	std::io::stdout().flush().unwrap();
	std::io::stdin().read_line(input).unwrap();
}
fn save_videos(videos: Vec<&str>) -> std::io::Result<()>
{
	let mut number_save: String = String::new();
	let msg: String = format!("Enter the video you want to save (1-{}): ", videos.len() + 1);
	get_input(&msg, &mut number_save);
	match number_save.trim().parse::<u32>()
	{
		Ok(n) => {
			let file_name: &str = "videos.txt";
			let mut file: File = OpenOptions::new()
				.create(true)   // Create the file if it doesn't exist
				.append(true)   // Open for appending
				.open(file_name)?;

			writeln!(file, "{}", videos[n as usize + 1])?; // Write data with newline
			println!("Saved!");
		}
		Err(_) => {}
	}
	drop(number_save);
	Ok(())
}

fn get_random_number() -> i32
{
	rand::rng().random_range(1000..=9999)
}

async fn get_videos(api_key: String, number_of_videos: &str, format: &str) -> Result<SearchResponse, Error>
{
	let client: Client = Client::new();
	let url: &str = "https://www.googleapis.com/youtube/v3/search";

	let random_number: String = get_random_number().to_string() + "." + format;

	let mut params: HashMap<&str, &str> = HashMap::new();
	params.insert("part", "snippet");
	params.insert("q", &random_number);
	params.insert("key", &api_key);
	params.insert("type", "video");
	params.insert("maxResults", number_of_videos);

	let response: SearchResponse = client
		.get(url)
		.query(&params)
		.send()
		.await?
		.json::<SearchResponse>()
		.await?;
	drop(client);
	drop(params);
	drop(random_number);
	Ok(response)
}

async fn get_results(number_of_videos: &str, format: Option<&str>) -> Vec<String>
{
	let base_url: &str = "https://www.youtube.com/watch?v=";
	let api_key: String = env::var("API_KEY").expect("api_key not set");
	match get_videos(api_key, number_of_videos, format.unwrap_or("avi")).await
	{
		Ok(response) =>
			{
				let mut video_links: Vec<String> = Vec::new();
				for item in response.items
				{
					println!("Title: {}", item.snippet.title);
					println!("Video ID: {}", item.id.video_id);
					println!("Channel: {}", item.snippet.channel_title);
					println!("Description: {}\n", item.snippet.description);

					let complete_url: String = base_url.to_owned() + &*item.id.video_id;
					video_links.push(complete_url.clone());

					webbrowser::open(&complete_url).unwrap();
				}
				video_links
			}
		Err(err) => { 
			eprintln!("Error: {}", err);
			Vec::new()
		}
	}
}

#[tokio::main]
async fn main()
{
	dotenv().ok();
	let formats: [&str; 5] = ["avi", "mov", "wmv", "mpg", "mpeg"];
	let mut video_format_input: String = String::new();
	get_input("Enter the video format: ", &mut video_format_input);
	video_format_input = video_format_input.trim().to_ascii_lowercase();
	for format in formats
	{
		if format.contains(video_format_input.as_str())
		{
			let mut number_of_videos_input: String = String::new();
			get_input("Enter your amount of videos to open: ", &mut number_of_videos_input);
			match number_of_videos_input.trim().parse::<u32>()
			{
				Ok(_) => {
					let videos: Vec<String> = get_results(&number_of_videos_input, Some(format)).await;
					if videos.len() != 0
					{
						let vvideos: Vec<&str> = videos
							.iter()
							.map(|str| str.as_str())
							.collect::<Vec<&str>>();
						let mut save_videos_input: String = String::new();
						get_input("Do you want to save any video? (yes/no): ", &mut save_videos_input);
						match save_videos_input.trim().to_ascii_lowercase().as_str()
						{ 
							"y" | "yes" => {
								save_videos(vvideos).unwrap();
							}
							_ => return
						}
					}
				},
				Err(err) => eprintln!("Failed to parse number of videos: {}", err)
			}
			drop(number_of_videos_input);
		}
	}
	drop(video_format_input);
}