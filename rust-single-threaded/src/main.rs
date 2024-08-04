use html_macro::html;
use serde::Deserialize;
use std::{
    error::Error,
    fs::{read_to_string, File},
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
use virtual_node::*;

#[derive(Deserialize, Debug)]
struct Player {
    name: String,
    score: u32,
}

#[derive(Deserialize)]
struct PlayersJson {
    players: Vec<Player>,
}

fn read_players() -> Result<Vec<Player>, Box<dyn Error>> {
    let file = File::open("../players.json")?;
    let players_json: PlayersJson = serde_json::from_reader(BufReader::new(file))?;
    Ok(players_json.players)
}

fn player_node(player: &Player) -> VirtualNode {
    html! { <li class="my-2 ml-2 even:bg-white odd:bg-gray-50">{*player.name} {" - "} {player.score}</li> }
}

fn player_list_node(players: &Vec<Player>) -> VirtualNode {
    let nodes = players
        .iter()
        .map(player_node)
        .collect::<Vec<VirtualNode>>();
    html! { <ul class="border">{nodes}</ul> }
}

fn render_html(players: &Vec<Player>) -> String {
    let html: VirtualNode = html! {
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link href="/styles.css" rel="stylesheet">
            </head>
            <body class="container">
              <div class="ml-2">
                  <h1 class="text-xl my-4">Players from Rust App</h1>
                  {player_list_node(players)}
              </div>
            </body>
        </html>
    };

    format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8;\r\n\r\n{html}\r\n")
}

fn router(mut stream: &TcpStream) -> Option<String> {
    let request = BufReader::new(&mut stream).lines().next()?.ok()?;

    request.split_once(' ').and_then(|(verb, raw_route)| {
        raw_route
            .split_once(' ')
            .map(|(route, _)| format!("{verb} {route}"))
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let players = &read_players().unwrap();
    let styles = read_to_string("./public/styles.css")?;

    for stream in TcpListener::bind("localhost:8080")?.incoming() {
        let mut stream = stream?;
        if let Some(route) = router(&stream) {
            match route.as_str() {
                "GET /" => stream.write_all(render_html(players).as_bytes())?,
                "GET /styles.css" => stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8;\r\n\r\n{styles}\r\n").as_bytes())?,
                _ => {}
            }
        }
    }
    Ok(())
}
