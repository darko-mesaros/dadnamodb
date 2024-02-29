use serde_json;
use serde::{Serialize, Deserialize};
use anyhow::Error;
use uuid::Uuid;

use aws_sdk_dynamodb::types::AttributeValue;

use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use std::io::Write;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct DadJoke {
    pub joke: String,
    pub punchline: String,
    pub category: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub id: Uuid,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DadJokesFile {
    pub jokes: Vec<DadJoke>,
}

impl DadJoke {
    pub fn new(joke: String, punchline: String, category: String, id: Uuid) -> Self {
        DadJoke {
            joke,
            punchline,
            category,
            id,
        }
    }
    pub fn say(&self) {
        let punchline = &self.punchline.clone();
        println!("========================================");
        println!("{}", &self.joke);
        sleep(Duration::from_secs(2));
        for c in punchline.chars() {
            print!("{}", c);
            stdout().flush();
            sleep(Duration::from_millis(25));
        }
        println!(" ");
        println!("yeah ...");
        println!("========================================");

    }
}

impl From<&HashMap<String, AttributeValue>> for DadJoke {
    fn from(value: &HashMap<String, AttributeValue>) -> Self {
        // TODO: use try_parse properly for uuid
        let uuid = Uuid::parse_str((as_string(value.get("id"), &"".to_string())).as_str()).unwrap();
        let mut dadjoke = DadJoke::new(
            as_string(value.get("joke"), &"".to_string()),
            as_string(value.get("punchline"), &"".to_string()),
            as_string(value.get("category"), &"".to_string()),
            uuid,
        );
        dadjoke
    }
}

// required for dynamodb
fn as_string(val: Option<&AttributeValue>, default: &String) -> String {
    if let Some(v) = val {
        if let Ok(s) = v.as_s() {
            return s.to_owned();
        }
    }
    default.to_owned()
}

pub fn load_jokes_from_file<P: AsRef<Path>>(path: P) -> Result<DadJokesFile, Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut dj: DadJokesFile = serde_json::from_reader(reader)?;
    for joke in dj.jokes.iter_mut(){
        joke.id = Uuid::new_v4();
    }

    
    Ok(dj)
}
