use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct SearchResponse
{
	pub(crate) items: Vec<VideoItem>
}

#[derive(Debug, Deserialize)]
pub(crate) struct VideoItem
{
	pub(crate) id: VideoId,
	pub(crate) snippet: Snippet
}

#[derive(Debug, Deserialize)]
pub(crate) struct VideoId
{
	#[serde(rename = "videoId")]
	pub(crate) video_id: String
}

#[derive(Debug, Deserialize)]
pub(crate) struct Snippet
{
	pub(crate) title: String,
	pub(crate) description: String,
	#[serde(rename = "channelTitle")]
	pub(crate) channel_title: String
}