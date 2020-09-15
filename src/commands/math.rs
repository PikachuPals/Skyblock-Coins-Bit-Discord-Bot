use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

use serenity::utils::Colour;

use requests::ToJson;

use std::collections::HashMap;
use std::env;
use std::time::Instant;

use std::sync::{Arc, Mutex, RwLock};
use std::thread;

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

    let bits_item_cost_vec = vec![2000, 500, 3000, 300, 8000, 1200, 4000, 1500, 2000, 4000];
    let item_array: [String; 10] = ["God Potion".to_string(), "Kat Flower".to_string(), "Heat Core".to_string(), "Hyper Catalyst Upgrade".to_string(), "Ultimate Carrot Candy Upgrade".to_string(),
    "Colossal Experience Bottle Upgrade".to_string(), "Jumbo Backpack Upgrade".to_string(), "Minion Storage X-pender".to_string(), "Hologram".to_string(), "Enchanted Book".to_string()];

    let mut bits_item_vec = vec![];

    for (item, price) in bits_items_lowest_prices.iter(){
        let index = item_array.iter().position(|x| x == item).unwrap();
        bits_item_vec.push(BitsItemPrices::new(item, bits_item_cost_vec[index], *price));
    }

    let cookie_output = format!("*Booster Cookie Price:* `{}`\nItems are organised into highest coins per bit.\nﾠﾠ", buy_cookie_price);

    let mut output_fields_vec = vec![];

    bits_item_vec.sort_by(|a, b| b.coins_per_bit().cmp(&a.coins_per_bit()));

    for listing in bits_item_vec {
        output_fields_vec.push((format!("{:.15}", listing.bits_item),
                                format!("BIN: *{}*\n$/b: *{}*\nﾠﾠ", listing.lowest_cost, listing.coins_per_bit()),
                                true,));
    }


    let timed_search = format!("Completed in {:.2?}.", start.elapsed());

    let _msg = msg.channel_id.send_message(&ctx.http, |m|{
        m.content(timed_search);
        m.embed(|e| {
            e.title("");
            e.description(cookie_output);
            e.thumbnail("https://i.imgur.com/JNpxJ7I.png");
            e.colour(Colour::FOOYOO);
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

    let item_array = ["God Potion".to_string(), "Kat Flower".to_string(), "Heat Core".to_string(), "Hyper Catalyst Upgrade".to_string(), "Ultimate Carrot Candy Upgrade".to_string(),
    "Colossal Experience Bottle Upgrade".to_string(), "Jumbo Backpack Upgrade".to_string(), "Minion Storage X-pender".to_string(), "Hologram".to_string(), "Enchanted Book".to_string()];

    let lowest_prices = Arc::new(RwLock::new(HashMap::new()));

    for item in item_array.iter(){
        let mut map = lowest_prices.write().unwrap();
        map.insert(item.to_string(), Mutex::new(999999999));
        drop(map);
    }

    let item_array = Arc::new(RwLock::new(item_array));

    let mut handles = vec![];

    for i in 0i32..=auction_pages{

        let item_array = Arc::clone(&item_array);
        let lowest_prices = Arc::clone(&lowest_prices);
        let handle = thread::spawn(move || worker_thread(i, item_array, lowest_prices));
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }

    let mut return_hashmap: HashMap<String, i32> = HashMap::new();

    let lowest_prices_output = &*lowest_prices.read().expect("RwLock poisoned.");

    for (item, price) in lowest_prices_output {
        let number = *price.lock().unwrap();
        return_hashmap.insert(item.to_string(), number);
    }

    return return_hashmap;
}

fn worker_thread(i: i32, item_array: Arc<RwLock<[String; 10]>>, lowest_prices: Arc<RwLock<HashMap<String, Mutex<i32>>>>) {

    let item_array_mutex = item_array.read().expect("RwLock poisoned.");

    let mut page_auctions = String::from(" https://api.hypixel.net/skyblock/auctions?page=");
    let page_number = i.to_string();
    page_auctions.push_str(&page_number);

    let response = requests::get(page_auctions).unwrap();
    let data = response.json().unwrap();

    let ebook = "Enchanted Book".to_string();
    let expertise_enchant = "Expertise".to_string();

    for auc in data["auctions"].members(){

        for auc_item in item_array_mutex.iter() {

            if auc["item_name"].as_str().unwrap() == auc_item && auc["bin"].as_bool() != None {

                let low_price_mutex = lowest_prices.read().unwrap();
                let lowest_price_of_item = low_price_mutex.get(auc_item).unwrap();
                let mut number = *lowest_price_of_item.lock().unwrap();
                drop(low_price_mutex);
                let auc_item_price = auc["starting_bid"].as_i32().unwrap();

                if auc_item == &ebook && auc["item_lore"].as_str().unwrap().contains(&expertise_enchant) {
                    if auc_item_price < number {
                        let mut low_price_write = lowest_prices.write().unwrap();
                        number = auc_item_price;
                        low_price_write.insert(auc_item.to_string(), Mutex::new(number));
                        drop(low_price_write);
                    }
                }

                else if auc_item != &ebook {

                    if auc_item_price < number {
                        let mut low_price_write = lowest_prices.write().unwrap();
                        number = auc_item_price;
                        low_price_write.insert(auc_item.to_string(), Mutex::new(number));
                        drop(low_price_write);
                    }
                }
                drop(number)
            }
        }
    }
}
