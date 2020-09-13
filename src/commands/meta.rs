use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use requests::ToJson;

use std::collections::HashMap;

#[command]
fn stats(ctx: &mut Context, msg: &Message) -> CommandResult {

    let _ = msg.channel_id.say(&ctx.http, "Working...");
    let mut bits_item_map = HashMap::new();

    bits_item_map.insert(BitsItemPrices::new("God Potion", 2000), 1);
    bits_item_map.insert(BitsItemPrices::new("Kat Flower", 500), 1);
    bits_item_map.insert(BitsItemPrices::new("Heat Core", 3000), 1);
    bits_item_map.insert(BitsItemPrices::new("Hyper Catalyst Upgrade", 300), 1);
    bits_item_map.insert(BitsItemPrices::new("Ultimate Carrot Candy Upgrade", 8000), 1);
    bits_item_map.insert(BitsItemPrices::new("Colossal Experience Bottle Upgrade", 1200), 1);
    bits_item_map.insert(BitsItemPrices::new("Jumbo Backpack Upgrade", 4000), 1);
    bits_item_map.insert(BitsItemPrices::new("Minion Storage X-pender", 1500), 1);
    bits_item_map.insert(BitsItemPrices::new("Hologram", 2000), 1);

//    let mojang_response = requests::get("https://api.mojang.com/users/profiles/minecraft/PikachuPals").unwrap();
//    let mojang_data = mojang_response.json().unwrap();
//    let user_uuid = mojang_data["id"].as_str().unwrap();

//    let mut skyblock_request = String::from("https://api.hypixel.net/Skyblock/profiles?key=23056a2e-5590-4a1f-881a-51452c7723b5&uuid=");
//    skyblock_request.push_str(&user_uuid);

//    let response = requests::get(skyblock_request).unwrap();
//    let data = response.json().unwrap();

    let skyblock_auctions = String::from(" https://api.hypixel.net/skyblock/auctions?page=0");
    let response = requests::get(skyblock_auctions).unwrap();
    let data = response.json().unwrap();

    let auction_pages = data["totalPages"].as_i32().unwrap();

    let bits_items_lowest_prices = get_lowest_bin_values(auction_pages);

    for (item, price) in bits_items_lowest_prices.iter(){
        println!("{}'s lowest price is {}", item, price);
    }

    let _ = msg.channel_id.say(&ctx.http, "Finished");

    Ok(())
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct BitsItemPrices{
    bits_item: String,
    bits_cost: i32,
}

impl BitsItemPrices{
    fn new(bits_item: &str, bits_cost: i32) -> BitsItemPrices{
        BitsItemPrices {bits_item: bits_item.to_string(), bits_cost: bits_cost}
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
// 23056a2e-5590-4a1f-881a-51452c7723b5
