use color_eyre::eyre::{Context, Result};
use reqwest::Client;

use crate::projects;
use crate::sections;
use crate::tasks;
use crate::tasks::Task;

pub async fn fetch_projects(client: &Client) -> Result<Vec<projects::Project>> {
    let response = client
        .get("https://api.todoist.com/rest/v2/projects")
        .send()
        .await
        .context("Failed to send request to fetch projects")?;

    if response.status().is_client_error() {
        return Err(color_eyre::eyre::eyre!("Received a 400 error: {:?}. This would most likely be because of an incorrect token. Check your config file for token.", response.status()));
    }

    let response_text = response
        .text()
        .await
        .context("Failed to read response text")?;

    let serialized: Vec<projects::Project> = serde_json::from_str(&response_text)
        .context("Failed to deserialize response into Vec<Project>")?;
    Ok(serialized)
}

pub async fn fetch_tasks(
    client: &reqwest::Client,
) -> Result<Vec<tasks::Task>, Box<dyn std::error::Error>> {
    let response = client
        .get("https://api.todoist.com/rest/v2/tasks")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let serialized: Vec<tasks::Task> = serde_json::from_str(&response).unwrap();
    Ok(serialized)
}

pub async fn fetch_sections(
    client: &reqwest::Client,
) -> Result<Vec<sections::Section>, Box<dyn std::error::Error>> {
    let response = client
        .get("https://api.todoist.com/rest/v2/sections")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let serialized: Vec<sections::Section> = serde_json::from_str(&response).unwrap();
    Ok(serialized)
}

pub async fn update_task(
    client: &reqwest::Client,
    json: serde_json::Value,
    task_id: String,
    tx: std::sync::mpsc::Sender<Task>,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .post(format!("https://api.todoist.com/rest/v2/tasks/{}", task_id))
        .json(&json)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let serialized: Task = serde_json::from_str(&response).unwrap();
    tx.send(serialized).unwrap();
    Ok(())
}

pub async fn close_task(
    client: &reqwest::Client,
    task_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.todoist.com/rest/v2/tasks/{}/close", task_id);

    let _response = client.post(url).send().await?;

    Ok(())
}

pub async fn create_task<'a>(
    client: &reqwest::Client,
    json: serde_json::Value,
    tx: std::sync::mpsc::Sender<Task>,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .post("https://api.todoist.com/rest/v2/tasks")
        .json(&json)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let serialized: Task = serde_json::from_str(&response).unwrap();
    tx.send(serialized).unwrap();
    Ok(())
}
