use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

use requests::ToJson;

use std::env;

#[command]
pub fn multiply(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>().unwrap();
    let two = args.single::<f64>().unwrap();

    let product = one * two;

    let _ = msg.channel_id.say(&ctx.http, product);

    Ok(())
}

#[command]
pub fn cookie(ctx: &mut Context, msg: &Message) -> CommandResult {

    let hypixel_token = env::var("HYPIXEL_TOKEN")
        .expect("Expected hypixel token in the environment");

    let skyblock_bazaar_cookie = format!("https://api.hypixel.net/skyblock/bazaar?key={}", hypixel_token);
    let response = requests::get(skyblock_bazaar_cookie).unwrap();
    let data = response.json().unwrap();

    let buy_cookie_price = &data["products"]["BOOSTER_COOKIE"]["sell_summary"][0]["pricePerUnit"];

    let _ = msg.channel_id.say(&ctx.http, buy_cookie_price);

    Ok(())
}
