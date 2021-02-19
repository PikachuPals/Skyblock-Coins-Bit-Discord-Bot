use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

use serenity::utils::Colour;

use requests::ToJson;

use std::collections::HashMap;
use std::time::Instant;
use std::sync::mpsc;
use std::thread;
use std::sync::mpsc::Sender;

#[command]
pub async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let one = args.single::<f64>()?;
    let two = args.single::<f64>()?;

    let product = one * two;

    msg.channel_id.say(&ctx.http, product).await?;

    Ok(())
}

const ITEMS: &[&str; 24] = &["Ender Artifact", "Wither Artifact", "Hegemony Artifact",
"Sharpness VI", "Giant Killer VI", "Power VI", "Growth VI", "Protection VI",
"Sharpness VII", "Giant Killer VII", "Power VII", "Growth VII", "Protection VII", "Counter-Strike V",
"Big Brain III", "Vicious III",
"Parrot Legendary", "Parrot Epic", "Turtle Legendary", "Turtle Epic", "Jellyfish Legendary", "Jellyfish Epic",
"Travel Scroll to Dark Auction", "Plasma Nucleus"];

const CATEGORIES: &[&str; 5] = &["Artifacts", "Books","Book Bundles", "Pets", "Misc"];
const CATEGORIES_COUNT: [i32; 5] = [3, 11, 2, 6, 2];

#[command]
pub async fn da(ctx: &Context, msg: &Message) -> CommandResult {

  let start = Instant::now();

  msg.channel_id.say(&ctx.http, "Working...").await?;

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

  let dark_auction_lowest_prices = get_lowest_bin_values(auction_pages);

  let mut item_prices: HashMap<String, String> = HashMap::new();

  for item in ITEMS.iter(){
      item_prices.insert(item.to_string() ,format!("$: {}", *dark_auction_lowest_prices.get(&item.to_string()).unwrap()));
  }

  let mut output_fields_vec = Vec::with_capacity(32);
  let mut output_fields_vec2 = Vec::with_capacity(32);

  let mut count = 0;
  let mut category = 0;
  let mut input_count = 0;

  output_fields_vec.push((format!("ﾠﾠ\n__{}__", CATEGORIES[0]),
                        "ﾠﾠ",
                        false,));

for item in ITEMS.iter(){

    if input_count < 24 {
        if CATEGORIES_COUNT[category] > count {
                count += 1;
            }

        else {

            category += 1;
            output_fields_vec.push((format!("ﾠﾠ\n__{}__", CATEGORIES[category]),
                                        "ﾠﾠ",
                                        false,));
            count = 1;
            input_count += 1;
        }

        output_fields_vec.push((format!("{}", item),
        item_prices.get(&item.to_string()).unwrap(),
        true,));
        input_count += 1;
    }

    else {

        if CATEGORIES_COUNT[category] > count {
                count += 1;
            }

        else {

            category += 1;
            output_fields_vec2.push((format!("ﾠﾠ\n__{}__", CATEGORIES[category]),
                                        "ﾠﾠ",
                                        false,));
            count = 1;
            input_count += 1;
        }

        output_fields_vec2.push((format!("{}", item),
        item_prices.get(&item.to_string()).unwrap(),
        true,));
        input_count += 1;
    }
}

  let timed_search = format!("Completed in {:.2?}.", start.elapsed());

  msg.channel_id.send_message(&ctx.http, |m|{
      m.content(timed_search);
      m.embed(|e| {
          e.title("`Dark Auction BIN Prices`");
          e.description("");
          e.thumbnail("https://i.imgur.com/JNpxJ7I.png");
          e.colour(Colour::FOOYOO);
          e.fields(output_fields_vec);
          e });
      m
  }).await?;

  msg.channel_id.send_message(&ctx.http, |m|{
      m.embed(|e| {
          e.colour(Colour::FOOYOO);
          e.fields(output_fields_vec2);
          e });
      m
  }).await?;


  Ok(())
}

fn get_lowest_bin_values(auction_pages: i32) ->  HashMap<String, i32>{

  let mut lowest_prices: HashMap<String, i32> = HashMap::new();

  for item in ITEMS.iter(){
      lowest_prices.insert(item.to_string(), 999999999);
  }

  let mut sender_vector: Vec<Sender<i32>> = Vec::with_capacity(ITEMS.len());
  let mut receiver_vector = Vec::with_capacity(ITEMS.len());

  for _n in 0..ITEMS.len(){
      let (tx, rx) = mpsc::channel();
      sender_vector.push(tx);
      receiver_vector.push(rx);
  }

  let mut handles = vec![];

  let mut threads_pages: Vec<i32> = vec![0];
  let threads: i32 = 8;
  let pages_per_thread: i32 = auction_pages / threads;
  let rem_pages: i32 = auction_pages % threads;
  for thread in 1..=threads{
      if thread != threads{
          threads_pages.push(thread * pages_per_thread);
      }
      else{
          threads_pages.push((thread * pages_per_thread) + rem_pages);
      }
  }

  for i in 0..threads_pages.len() - 1 {
      let mut sender_vector_clone: Vec<Sender<i32>> = Vec::with_capacity(ITEMS.len());
      let start_page = threads_pages[i].clone();
      let end_page = threads_pages[i + 1].clone();
      for tx in &sender_vector{
          let tx_clone = tx.clone();
          sender_vector_clone.push(tx_clone);
      }
      let handle = thread::spawn(move || work_thread(sender_vector_clone,
           start_page, end_page));
      handles.push(handle);
  }

  for handle in handles{
      handle.join().unwrap();
  }

  for sender in sender_vector{
      drop(sender);
  }

  for item in ITEMS.iter() {
      let index = ITEMS.iter().position(|x| x == item).unwrap();

      for price in &receiver_vector[index]{
          if price < *lowest_prices.get(&item.to_string()).unwrap() {
              lowest_prices.insert(item.to_string(), price);
          }
      }
  }

  return lowest_prices;
}

fn work_thread(sender_vector: Vec<Sender<i32>>, i: i32, e: i32){

  for page in i..e{
      let mut page_auctions = String::from(" https://api.hypixel.net/skyblock/auctions?page=");
      let page_number = page.to_string();
      page_auctions.push_str(&page_number);

      let response = requests::get(page_auctions).unwrap();
      let data = response.json().unwrap();

      let ebook = "Enchanted Book".to_string();
      let enchants: [String; 11] = ["Sharpness VI".to_string(), "Giant Killer VI".to_string(), "Power VI".to_string(), "Growth VI".to_string(), "Protection VI".to_string(),
      "Sharpness VII".to_string(), "Giant Killer VII".to_string(), "Power VII".to_string(), "Growth VII".to_string(), "Protection VII".to_string(), "Counter-Strike V".to_string()];

      let unwanted_enchants: [String; 3] = ["Fire".to_string(), "Projectile".to_string(), "Blast".to_string()];
      let mut fake_enchant;

      let ebundle = "Enchanted Book Bundle".to_string();
      let bundles: [String; 2] = ["Big Brain III".to_string(), "Vicious III".to_string()];

      for auc in data["auctions"].members(){
          for auc_item in ITEMS.iter() {
              if auc["bin"].as_bool() != None{
                  if auc["item_name"].as_str().unwrap() == *auc_item && auc["bin"].as_bool() != None {
                      let index = ITEMS.iter().position(|x| x == auc_item).unwrap();
                      let auc_item_price = auc["starting_bid"].as_i32().unwrap();
                      sender_vector[index].send(auc_item_price).unwrap();
                  }

                  else if auc["item_name"].as_str().unwrap() == &ebook {
                      for enchant in enchants.iter() {

                          if auc["item_lore"].as_str().unwrap().contains(enchant){

                              if auc["item_lore"].as_str().unwrap().contains(&"Protection".to_string()){
                                  fake_enchant = false;
                                  for unwanted_enchant in unwanted_enchants.iter(){
                                      if auc["item_lore"].as_str().unwrap().contains(unwanted_enchant) {
                                          fake_enchant = true;
                                      }
                                  }
                                  if !fake_enchant{
                                      let index = ITEMS.iter().position(|x| x == enchant).unwrap();
                                      let auc_item_price = auc["starting_bid"].as_i32().unwrap();
                                      sender_vector[index].send(auc_item_price).unwrap();
                                  }
                              }

                              else{
                                  let index = ITEMS.iter().position(|x| x == enchant).unwrap();
                                  let auc_item_price = auc["starting_bid"].as_i32().unwrap();
                                  sender_vector[index].send(auc_item_price).unwrap();
                              }
                          }
                      }
                  }

                  else if auc["item_name"].as_str().unwrap() == &ebundle {
                      for bundle in bundles.iter() {
                          if auc["item_lore"].as_str().unwrap().contains(bundle){
                              let index = ITEMS.iter().position(|x| x == bundle).unwrap();
                              let auc_item_price = auc["starting_bid"].as_i32().unwrap();
                              sender_vector[index].send(auc_item_price).unwrap();
                          }
                      }
                  }

                  else if auc_item.contains(&"Epic".to_string()) || auc_item.contains(&"Legendary".to_string()) {
                      let mut pet = auc_item.split_whitespace();
                      if auc["item_name"].as_str().unwrap().contains(pet.next().unwrap()) && auc["item_lore"].as_str().unwrap().contains(&pet.next().unwrap().to_uppercase()){
                          let index = ITEMS.iter().position(|x| x == auc_item).unwrap();
                          let auc_item_price = auc["starting_bid"].as_i32().unwrap();
                          sender_vector[index].send(auc_item_price).unwrap();
                      }
                  }

              }
          }
      }
  }
}
