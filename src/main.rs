extern crate openssl_probe;
extern crate reqwest;
extern crate serde;
extern crate typemap;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serenity;

use typemap::Key;
use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::framework::standard::StandardFramework;
use std::env;

const BASE_URL: &str = "https://wy59x9hce3.execute-api.ap-northeast-1.amazonaws.com/Prod/search";


#[derive(Debug,Deserialize)]
struct Source{
    url: String,
    title: String,
    image: String,
    descriptions: Vec<String>,
    tags: Vec<String>
}

#[derive(Debug,Deserialize)]
struct HitInner{
    #[serde(rename="_source")]
    source: Source
}

#[derive(Debug,Deserialize)]
struct Hit{
    hits: Vec<HitInner>  
}

#[derive(Debug,Deserialize)]
struct Response{
    hits: Hit
}

struct HttpClient;

impl Key for HttpClient{
    type Value = reqwest::Client;
}

struct Handler;

impl EventHandler for Handler{
    fn ready(&self, ctx: Context, _: Ready){
        ctx.set_game_name("いらすとや");
    }
}


command!(irasutoya(ctx, msg, args){
    let name = args.single::<String>().unwrap();
    let data = ctx.data.lock();
    let client = data.get::<HttpClient>().unwrap();
    let mut resp = client.get(BASE_URL)
        .query(&[("keyword", name)])
        .send().unwrap();
    if resp.status().is_success() {
        if let Ok(response) = resp.json::<Response>(){
            if !&response.hits.hits.is_empty(){
                if let Err(why) = msg.channel_id.send_message(|m| m
                    .embed(|e| e
                        .title(&response.hits.hits[0].source.title)
                        .description(&response.hits.hits[0].source.descriptions[0])
                        .image(&response.hits.hits[0].source.image)
                        .url(&response.hits.hits[0].source.url))) {
                    println!("Error sending message: {:?}", why);
                }
            }else{
                if let Err(why) = msg.channel_id.say("見つかりませんでした。"){
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }
});

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let token = env::var("TOKEN")
        .expect("Expected a token in the environment");

    let mut client = serenity::Client::new(&token, Handler).expect("Error in initializing client");

    {
        let mut data = client.data.lock();
        data.insert::<HttpClient>(reqwest::Client::new());
    }

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .unrecognised_command(|_, _, unknown_command_name| {
            println!("Could not find command named '{}'", unknown_command_name);
        })
        .command("irasutoya", |c| c.cmd(irasutoya)));

    if let Err(why) = client.start_autosharded() {
        println!("Client error: {:?}", why);
    }
}
