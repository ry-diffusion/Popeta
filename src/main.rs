#![feature(never_type)]
use anyhow::bail;
use anyhow::{Context, Result};
use azalea::{prelude::*, NoState};
use std::fs::read_to_string;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::atomic::{AtomicBool, Ordering};

static HAVE_OP: AtomicBool = AtomicBool::new(true);

fn load_conf(username: &mut String, server_addr: &mut String) -> Result<()> {
    let contents = read_to_string("./conf.txt").context("please create conf.txt")?;
    for line in contents.split('\n') {
        if line.len() < 3 {
            continue;
        }

        let (key, mut value) = line.split_once('=').context("invalid line")?;
        value = value.trim();
        match key {
            // Eu sei que isso é uma gambiarra, não me julge!
            "nick" => username.extend(value.chars()),
            "server_addr" => server_addr.extend(value.chars()),
            // Hacks são legais HAHAHAHA
            "have_op" => match value {
                "yes" | "no" => {
                    HAVE_OP.swap(value == "yes", Ordering::Relaxed);
                }
                _ => bail!("invalid value for have_op!"),
            },
            _ => {}
        }
    }

    Ok(())
}

#[tokio::main]
// Main never returns? WTF!
// or it is just screaming
async fn main() -> Result<!> {
    let mut username = String::new();
    let mut server_addr = String::new();

    load_conf(&mut username, &mut server_addr).context("unable to parse config file")?;

    println!("username={username} server_addr={server_addr}");
    let account = Account::offline(&username);

    ClientBuilder::new()
        .set_handler(handle)
        .start(
            account,
            server_addr
                .to_socket_addrs()
                .context("unable to resolve addr")?
                .next()
                .context("invalid addr")?,
        )
        .await
        .context("Runtime error")
}

async fn handle(bot: Client, event: Event, _state: NoState) -> anyhow::Result<()> {
    match event {
        Event::Chat(m) => {
            let Some(remetent) = m.username() else {
                return Ok(());
            };

            let contents = m.content();

            let mut parser = contents.split_whitespace();

            // WHAT THE FUCK? Sim, eu gosto de let-else, obrigado.
            let Some(".popeta") = parser.next() else {
                return Ok(());
            };

            let Some(command) = parser.next() else {
                return Ok(());
            };

            match command {
                "farm" if HAVE_OP.load(Ordering::Relaxed) => {
                    bot.send_command_packet(&format!("tp {} {remetent}", bot.profile.name));
                    bot.send_command_packet(&format!("w {remetent} Pronto!"))
                }

                _ => {}
            }

            println!(":: {}", m.message().to_ansi());
        }

        Event::Login => {
            bot.chat("Olá! Eu sou o Popeta, o bot legal. Se você gostou desse projeto, dê uma star no GitHub por favor: https://github.com/Ry-Diffusion/Popeta");
        }

        Event::Disconnect(why) => {
            eprintln!("bot disconnected! why? {why:?}");
            bot.disconnect();
            println!("bot:disconnected")
        }

        _ => {}
    }
    Ok(())
}
