use color_eyre::eyre::{Context, Result};
use reqwest::Client;

use crate::projects;
use crate::sections;
use crate::tasks;
use crate::tasks::Task;
use crate::TaskResult;

// 200 is the largest page the Unified API v1 accepts; larger values return an
// empty result set instead of an error.
const PAGE_LIMIT: u32 = 200;

fn next_cursor(parsed_json: &serde_json::Value) -> Option<String> {
    parsed_json
        .get("next_cursor")
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
}

pub async fn fetch_projects(client: &Client) -> Result<Vec<projects::Project>> {
    let mut all_projects: Vec<projects::Project> = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let mut request = client
            .get("https://api.todoist.com/api/v1/projects")
            .query(&[("limit", PAGE_LIMIT)]);

        if let Some(ref cursor) = cursor {
            request = request.query(&[("cursor", cursor)]);
        }

        let response = request
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

        let parsed_json: serde_json::Value = serde_json::from_str(&response_text)
            .context("Failed to parse JSON")?;
        let array_data = parsed_json.get("results").unwrap_or(&parsed_json);

        let serialized: Vec<projects::Project> = serde_json::from_value(array_data.clone())
            .context("Failed to deserialize response into Vec<Project>")?;
        all_projects.extend(serialized);

        cursor = next_cursor(&parsed_json);
        if cursor.is_none() {
            break;
        }
    }

    Ok(all_projects)
}

pub async fn fetch_tasks(
    client: &reqwest::Client,
) -> Result<Vec<tasks::Task>, Box<dyn std::error::Error>> {
    let mut all_tasks: Vec<tasks::Task> = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let mut request = client
            .get("https://api.todoist.com/api/v1/tasks")
            .query(&[("limit", PAGE_LIMIT)]);

        if let Some(ref cursor) = cursor {
            request = request.query(&[("cursor", cursor)]);
        }

        let response = request.send().await?.text().await?;

        let parsed_json: serde_json::Value = serde_json::from_str(&response)?;
        let array_data = parsed_json.get("results").unwrap_or(&parsed_json);

        let serialized: Vec<tasks::Task> = serde_json::from_value(array_data.clone())?;
        all_tasks.extend(serialized);

        cursor = next_cursor(&parsed_json);
        if cursor.is_none() {
            break;
        }
    }

    Ok(all_tasks)
}

pub async fn fetch_sections(
    client: &reqwest::Client,
) -> Result<Vec<sections::Section>, Box<dyn std::error::Error>> {
    let mut all_sections: Vec<sections::Section> = Vec::new();
    let mut cursor: Option<String> = None;

    loop {
        let mut request = client
            .get("https://api.todoist.com/api/v1/sections")
            .query(&[("limit", PAGE_LIMIT)]);

        if let Some(ref cursor) = cursor {
            request = request.query(&[("cursor", cursor)]);
        }

        let response = request.send().await?.text().await?;

        let parsed_json: serde_json::Value = serde_json::from_str(&response)?;
        let array_data = parsed_json.get("results").unwrap_or(&parsed_json);

        let serialized: Vec<sections::Section> = serde_json::from_value(array_data.clone())?;
        all_sections.extend(serialized);

        cursor = next_cursor(&parsed_json);
        if cursor.is_none() {
            break;
        }
    }

    Ok(all_sections)
}

pub async fn update_task(
    client: &reqwest::Client,
    json: serde_json::Value,
    task_id: String,
    tx: std::sync::mpsc::Sender<TaskResult>,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = match client
        .post(format!("https://api.todoist.com/api/v1/tasks/{}", task_id))
        .json(&json)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            let error_msg = format!("Network error: {}", e);
            tx.send(TaskResult::Error(error_msg)).unwrap();
            return Err(e.into());
        }
    };

    let response_ref = &response;

    match response_ref.error_for_status_ref() {
        Ok(resp) => resp,
        Err(e) => {
            let status_code = e.status().unwrap_or_default();
            let response_text = response.text().await.unwrap_or_default();
            let error_msg = format!("API error: {} ({}) \n\n {}", status_code, e, response_text);
            tx.send(TaskResult::Error(error_msg)).unwrap();
            return Err(e.into());
        }
    };

    let response_text = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            let error_msg = format!("Failed to read response: {}", e);
            tx.send(TaskResult::Error(error_msg)).unwrap();
            return Err(e.into());
        }
    };

    match serde_json::from_str::<Task>(&response_text) {
        Ok(serialized) => {
            tx.send(TaskResult::Task(serialized)).unwrap();
            Ok(())
        }
        Err(e) => {
            let error_msg = format!(
                "Failed to parse response: {} (Response was: {})",
                e, response_text
            );
            tx.send(TaskResult::Error(error_msg)).unwrap();
            Err(e.into())
        }
    }
}

pub async fn close_task(
    client: &reqwest::Client,
    task_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.todoist.com/api/v1/tasks/{}/close", task_id);

    let _response = client.post(url).send().await?;

    Ok(())
}

pub async fn delete_task(
    client: &reqwest::Client,
    task_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://api.todoist.com/api/v1/tasks/{}", task_id);

    let _response = client.delete(url).send().await?;

    Ok(())
}

pub async fn create_task<'a>(
    client: &reqwest::Client,
    json: serde_json::Value,
    tx: std::sync::mpsc::Sender<TaskResult>,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .post("https://api.todoist.com/api/v1/tasks")
        .json(&json)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let serialized: Task = serde_json::from_str(&response).unwrap();
    tx.send(TaskResult::Task(serialized)).unwrap();
    Ok(())
}
