use reqwest::header::AUTHORIZATION;

use crate::projects;
use crate::tasks;

pub async fn fetch_projects() -> Result<Vec<projects::Project>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.todoist.com/rest/v2/projects")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", "afe0921da7503038a0597511a26a479498c5fd56"),
        )
        .send()
        .await?
        .text()
        .await?;

    let serialized: Vec<projects::Project> = serde_json::from_str(&response)?;

    // println!("response = {:#?}", serialized);
    Ok(serialized)
}

pub async fn fetch_tasks() -> Result<Vec<tasks::Task>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://api.todoist.com/rest/v2/tasks")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", "afe0921da7503038a0597511a26a479498c5fd56"),
        )
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // println!("RESPONSE {}", response);

    let serialized: Vec<tasks::Task> = serde_json::from_str(&response).unwrap();

    // println!("response = {:#?}", serialized);
    Ok(serialized)
}
