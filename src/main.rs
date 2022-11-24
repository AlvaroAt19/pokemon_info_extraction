use serde_json::Value;
use std::error::Error;
use csv;
use serde::Serialize;
use std::fs::File;


#[derive(Debug, Serialize)]
struct Pokemon {
    name: String,
    weight: u16,
    type1: String,
    type2: Option<String>,
    hp:u16,
    attack:u16,
    defense:u16,
    special_attack:u16,
    special_defense:u16,
    speed:u16
}

impl Pokemon {
    fn new(name:String,weight: u16, type1: String, type2: Option<String>, hp:u16, attack:u16, defense:u16, special_attack:u16, special_defense:u16, speed:u16) ->Pokemon{
        return Pokemon{name:name,weight:weight,type1:type1,type2:type2,hp:hp,attack:attack,defense:defense,special_attack:special_attack,special_defense:special_defense,speed:speed}
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //Create a file to write csv data
    let  _file = File::create("pokemon.csv")?;
    //Writes to csv
    let mut wtr = csv::Writer::from_path("pokemon.csv")?;
    for i in 1..=151 {
        let pkm: Pokemon = get_pokemon(i).await.unwrap();
        wtr.serialize(pkm)?;
    };
    wtr.flush()?;
    Ok(())
}


async fn get_pokemon(i:u8) -> Result<Pokemon, reqwest::Error>{
    //Create a Pokemon
    let mut pkm:Pokemon = Pokemon::new(String::new(),0,String::new(),None,0,0,0,0,0,0);
    //Get API
    let content: String = reqwest::get(format!("https://pokeapi.co/api/v2/pokemon/{}/",i))
        .await?
        .text()
        .await?;
    //Convert the response to Json
    let content: Value = serde_json::from_str(&content).unwrap();

    //Get the attributes of requested pokemon and put into pkm
    pkm.name = content["name"].as_str().unwrap().to_owned();
    pkm.weight = content["weight"].as_u64().unwrap() as u16;
    pkm.type1 = content["types"][0]["type"]["name"].as_str().unwrap().to_owned();
    // Not all pokemon have a 2° type, so I check and return None if it list.len() < 2
    pkm.type2 = match content["types"].as_array().unwrap().len(){
        2 => Some(content["types"][1]["type"]["name"].as_str().unwrap().to_owned()),
        _ => None
    };
    pkm.hp = content["stats"][0]["base_stat"].as_u64().unwrap() as u16;
    pkm.attack = content["stats"][0]["base_stat"].as_u64().unwrap() as u16;
    pkm.defense = content["stats"][1]["base_stat"].as_u64().unwrap() as u16;
    pkm.special_attack = content["stats"][2]["base_stat"].as_u64().unwrap() as u16;
    pkm.special_defense = content["stats"][3]["base_stat"].as_u64().unwrap() as u16;
    pkm.speed = content["stats"][4]["base_stat"].as_u64().unwrap() as u16;
    //Return pkm
    Ok(pkm)

}
