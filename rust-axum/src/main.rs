use std::{error::Error, fs::File, io::BufReader, sync::Arc};

use axum::{extract::State, response::{Html, IntoResponse}, routing::get, Router};
use html_macro::html;
use serde::Deserialize;

use tokio::net::TcpListener;
use virtual_node::*;

#[derive(Deserialize, Debug, Clone)]
struct Player {
    name: String,
    score: u32,
}

#[derive(Deserialize)]
struct PlayersJson {
    players: Vec<Player>,
}

/// Return Arc so we don't clone the whole vector, but instead just the pointer reference
fn read_players() -> Result<Arc<Vec<Player>>, Box<dyn Error>> {
    let file = File::open("../players.json")?;
    let players_json: PlayersJson = serde_json::from_reader(BufReader::new(file))?;
    Ok(Arc::new(players_json.players))
}

fn player_node(player: &Player) -> VirtualNode {
    html! { <li class="my-2 ml-2 even:bg-white odd:bg-gray-50">{*player.name} {" - "} {player.score}</li> }
}

fn player_list_node(players: &[Player]) -> VirtualNode {
    let nodes = players
        .iter()
        .map(player_node)
        .collect::<Vec<VirtualNode>>();
    html! { <ul class="border">{nodes}</ul> }
}

async fn render_html(State(players): State<Arc<Vec<Player>>>) -> impl IntoResponse {
    let html_nodes = html! {
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link href="/styles.css" rel="stylesheet">
            </head>
            <body class="container">
              <div class="ml-2">
                  <h1 class="text-xl my-4">Players from Rust App</h1>
                  {player_list_node(&players)}
              </div>
            </body>
        </html>
    };

    Html(html_nodes.to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/", get(render_html)).with_state(read_players()?);
    axum::serve(TcpListener::bind("127.0.0.1:8080").await?, app).await?;
    Ok(())
}
