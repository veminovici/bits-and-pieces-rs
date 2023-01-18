use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Repos {
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    user: User,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    repositories: Repositories,
}

#[derive(Serialize, Deserialize)]
pub struct Repositories {
    nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize)]
pub struct Node {
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    println!("Hello, world!");

    let octocrab = octocrab::OctocrabBuilder::new().personal_token("---------".to_string()).build().map_err(|e| println!("ERR0: {e}"))?;

    let query = "query { 
        user(login: \"veminovici\") {
          repositories(first: 50) {
            nodes {
              name,
            }
          }
        }
      }";

    let response: serde_json::Value = octocrab.graphql(query).await.map_err(|e|println!("ERR1: {e}"))?;
    let repos = Repos::deserialize(response).map_err(|e|println!("ERR2: {e}"))?;
    let repositories = repos.data.user.repositories;
    for node in repositories.nodes {
        let nm = node.name;
        println!("Repository: {nm}");

    }

    Ok(())
}
