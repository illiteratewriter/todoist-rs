use reqwest::header::AUTHORIZATION;

use crate::projects;
use crate::sections;
use crate::tasks;
use crate::tasks::Task;

pub async fn fetch_projects(
    client: &reqwest::Client,
) -> Result<Vec<projects::Project>, Box<dyn std::error::Error>> {
    let response = client
        .get("https://api.todoist.com/rest/v2/projects")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", "31bd6a4adbba5480e76be2f2ce09dd53dc7ac3e7"),
        )
        .send()
        .await?
        .text()
        .await?;

    let serialized: Vec<projects::Project> = serde_json::from_str(&response)?;

    // println!("response = {:#?}", serialized);
    Ok(serialized)
}

pub async fn fetch_tasks(
    client: &reqwest::Client,
) -> Result<Vec<tasks::Task>, Box<dyn std::error::Error>> {
    let response = client
        .get("https://api.todoist.com/rest/v2/tasks")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", "31bd6a4adbba5480e76be2f2ce09dd53dc7ac3e7"),
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

pub async fn fetch_sections(
    client: &reqwest::Client,
) -> Result<Vec<sections::Section>, Box<dyn std::error::Error>> {
    let response = client
        .get("https://api.todoist.com/rest/v2/sections")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", "31bd6a4adbba5480e76be2f2ce09dd53dc7ac3e7"),
        )
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // println!("RESPONSE {}", response);

    let serialized: Vec<sections::Section> = serde_json::from_str(&response).unwrap();

    // println!("response = {:#?}", serialized);
    Ok(serialized)
}

pub async fn update_task(
    client: &reqwest::Client,
    task: Task,
) -> Result<(), Box<dyn std::error::Error>> {
    let task_string = serde_json::to_string(&task)?;
    let json: serde_json::Value = serde_json::from_str(&task_string)?;

    let _response = client
        .post(format!("https://api.todoist.com/rest/v2/tasks/{}", task.id))
        .header(
            AUTHORIZATION,
            format!("Bearer {}", "31bd6a4adbba5480e76be2f2ce09dd53dc7ac3e7"),
        )
        .json(&json)
        .send()
        .await?;
    Ok(())
}

pub async fn close_task(
    client: &reqwest::Client,
    task_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // println!("task id {}", task_id);
    let url = format!("https://api.todoist.com/rest/v2/tasks/{}/close", task_id);
    let _response = client
        .post(url)
        .header(
            AUTHORIZATION,
            format!("Bearer {}", "31bd6a4adbba5480e76be2f2ce09dd53dc7ac3e7"),
        )
        .send()
        .await?;
    Ok(())
}

pub async fn create_task<'a>(
    client: &reqwest::Client,
    json: serde_json::Value,
    tx: std::sync::mpsc::Sender<Task>,
) -> Result<(), Box<dyn std::error::Error>> {
    let response = client
        .post("https://api.todoist.com/rest/v2/tasks")
        .header(
            AUTHORIZATION,
            format!("Bearer {}", "31bd6a4adbba5480e76be2f2ce09dd53dc7ac3e7"),
        )
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
