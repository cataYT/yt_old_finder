use crate::{detail::SearchResponse};
use dotenv::dotenv;
use rand::Rng;
use reqwest::{
	Client,
	Error
};
use std::{
	array,
	collections::HashMap,
	env,
	fs::{File, OpenOptions},
	io::Write
};

pub(crate) struct Finder
{
	api_key: String,
	end_point: String,
	videos: Vec<String>,
	formats: [String; 5]
}

impl Finder
{
	pub fn new() -> Self
	{
		dotenv().ok();
		const FORMATS_STR: [&str; 5] = ["avi", "mov", "wmv", "mpg", "mpeg"];
		Finder
		{
			api_key: env::var("API_KEY").expect("api_key not set"),
			end_point: "https://www.googleapis.com/youtube/v3/search".to_string(),
			videos: Vec::new(),
			formats: array::from_fn(|i| FORMATS_STR[i].to_string())
		}
	}
	pub fn get_formats(&self) -> [String; 5]
	{
		self.formats.clone()
	}
	pub fn get_videos(&self) -> &Vec<String>
	{
		&self.videos
	}
	pub fn add_videos(&mut self, video_link: String)
	{
		self.videos.push(video_link);
	}

	pub fn save_videos(&self, n: Vec<usize>) -> std::io::Result<()>
	{
		let file_name: &str = "videos.txt";
		let mut file: File = OpenOptions::new()
			.create(true)
			.append(true)
			.open(file_name)?;
		
		for i in &n
		{
			writeln!(file, "{}", self.videos[*i - 1])?;
		}
		println!("Saved!");
		Ok(())
	}

	fn build_params(&self, format: &str, number_of_videos: &str) -> HashMap<String, String> 
	{
		let random_number: String = format!
		(
			"{}.{}",
			rand::rng().random_range(1000..=9999),
			format
		);

		let mut params: HashMap<String, String> = HashMap::new();
		params.insert("part".to_string(), "snippet".to_string());
		params.insert("q".to_string(), random_number);
		params.insert("key".to_string(), self.api_key.clone());
		params.insert("type".to_string(), "video".to_string());
		params.insert("maxResults".to_string(), number_of_videos.to_string());
		params
	}

	async fn get_response(&self, params: HashMap<String, String>) -> Result<SearchResponse, Error> 
	{
		let client: Client = Client::new();
		let response: SearchResponse = client
			.get(&self.end_point)
			.query(&params)
			.send()
			.await?
			.json::<SearchResponse>()
			.await?;

		Ok(response)
	}

	pub async fn query_videos(&self, number_of_videos: &str, format: Option<&str>) -> Result<SearchResponse, Error>
	{
		let params: HashMap<String, String> = self.build_params(format.unwrap_or("avi"), number_of_videos);
		let response: SearchResponse = self.get_response(params).await?;
		Ok(response)
	}
}