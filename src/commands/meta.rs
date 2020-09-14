use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use requests::ToJson;

use std::collections::HashMap;
use std::env;
use std::time::Instant;

#[command]
fn bits(ctx: &mut Context, msg: &Message) -> CommandResult {

    let hypixel_token = env::var("HYPIXEL_TOKEN")
        .expect("Expected hypixel token in the environment");

    let start = Instant::now();

    let _ = msg.channel_id.say(&ctx.http, "Working...");

    let skyblock_bazaar_cookie = format!("https://api.hypixel.net/skyblock/bazaar?key={}", hypixel_token);
    let response = requests::get(skyblock_bazaar_cookie).unwrap();
    let data = response.json().unwrap();

    let buy_cookie_price = &data["products"]["BOOSTER_COOKIE"]["sell_summary"][0]["pricePerUnit"];

//    let mojang_response = requests::get("https://api.mojang.com/users/profiles/minecraft/PikachuPals").unwrap();
//    let mojang_data = mojang_response.json().unwrap();
//    let user_uuid = mojang_data["id"].as_str().unwrap();

//    let mut skyblock_request = String::from("https://api.hypixel.net/Skyblock/profiles?key=...&uuid=");
//    skyblock_request.push_str(&user_uuid);

//    let response = requests::get(skyblock_request).unwrap();
//    let data = response.json().unwrap();

    let skyblock_auctions = String::from("https://api.hypixel.net/skyblock/auctions?page=0");
    let response = requests::get(skyblock_auctions).unwrap();
    let data = response.json().unwrap();

    let auction_pages = data["totalPages"].as_i32().unwrap();

    let bits_items_lowest_prices = get_lowest_bin_values(auction_pages);

    let bits_item_cost_vec = vec![2000, 500, 3000, 300, 8000, 1200, 4000, 1500, 2000];
    let item_array: [String; 9] = ["God Potion".to_string(), "Kat Flower".to_string(), "Heat Core".to_string(), "Hyper Catalyst Upgrade".to_string(), "Ultimate Carrot Candy Upgrade".to_string(),
    "Colossal Experience Bottle Upgrade".to_string(), "Jumbo Backpack Upgrade".to_string(), "Minion Storage X-pender".to_string(), "Hologram".to_string()];

    let mut bits_item_vec = vec![];

    for (item, price) in bits_items_lowest_prices.iter(){
        let index = item_array.iter().position(|x| x == item).unwrap();
        bits_item_vec.push(BitsItemPrices::new(item, bits_item_cost_vec[index], *price));
    }

    let cookie_output = format!("`Booster Cookie Price:` *{}*", buy_cookie_price);

    let mut output_fields_vec = vec![];

    for listing in bits_item_vec {
        output_fields_vec.push((format!("{:.15}", listing.bits_item),
                                format!("*BIN:* {}\n*$*/*b*: {}\nﾠﾠ", listing.lowest_cost, listing.coins_per_bit()),
                                true,));
    }


    let timed_search = format!("Completed in {:.2?}.", start.elapsed());

    let _msg = msg.channel_id.send_message(&ctx.http, |m|{
        m.content(timed_search);
        m.embed(|e| {
            e.title("");
            e.description(cookie_output);
            e.fields(output_fields_vec);
            e });
        m
    });

    Ok(())
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct BitsItemPrices{
    bits_item: String,
    bits_cost: i32,
    lowest_cost: i32,
}

impl BitsItemPrices{
    fn new(bits_item: &str, bits_cost: i32, lowest_cost: i32) -> BitsItemPrices{
        BitsItemPrices {bits_item: bits_item.to_string(), bits_cost: bits_cost, lowest_cost: lowest_cost}
    }

    fn coins_per_bit(&self) -> i32 {
        (self.lowest_cost / self.bits_cost).abs()
    }
}

fn get_lowest_bin_values(auction_pages: i32) ->  HashMap<String, i32>{

    let item_array: [String; 9] = ["God Potion".to_string(), "Kat Flower".to_string(), "Heat Core".to_string(), "Hyper Catalyst Upgrade".to_string(), "Ultimate Carrot Candy Upgrade".to_string(),
    "Colossal Experience Bottle Upgrade".to_string(), "Jumbo Backpack Upgrade".to_string(), "Minion Storage X-pender".to_string(), "Hologram".to_string()];

    let mut lowest_prices: HashMap<String, i32> = HashMap::new();

    for item in item_array.iter(){
        lowest_prices.insert(item.to_string(), 999999999);
    }

    for i in 0i32..=auction_pages{

        let mut page_auctions = String::from(" https://api.hypixel.net/skyblock/auctions?page=");
        let page_number = i.to_string();
        page_auctions.push_str(&page_number);

        let response = requests::get(page_auctions).unwrap();
        let data = response.json().unwrap();

        for auc in data["auctions"].members(){
            for auc_item in item_array.iter() {
                if auc["item_name"].as_str().unwrap() == auc_item && auc["bin"].as_bool() != None {

                    let lowest_price_of_item = lowest_prices.get(auc_item).unwrap();
                    let auc_item_price = auc["starting_bid"].as_i32().unwrap();

                    if auc_item_price < *lowest_price_of_item {
                        lowest_prices.insert(auc_item.to_string(), auc_item_price);
                    }
                }
            }
        }
    }

    return lowest_prices;
}
