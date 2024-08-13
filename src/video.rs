use serde::Deserialize;
use yew::Properties;

#[derive(Properties, PartialEq, Clone, Deserialize)]
pub struct Video {
    pub id: usize,
    pub title: String,
    pub speaker: String,
    pub url: String,
}
