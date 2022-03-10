use super::Error;
use super::{ request , _request};
use std::{
    fs,io,io::Write
};

/// The Image struct
#[derive(Debug)]
pub struct Image {
    pub url : Option<String>,
    pub bytes : bytes::Bytes
}

impl Image {
    /// Saves the image to a new file
    /// 
    /// # Example
    /// ```
    /// let meme = Meme::get("minecraft").unwrap();
    /// 
    /// let file = meme.image.save_as_new("meme.png");
    /// ```
    pub fn save_as_new(&self, path : &str) -> Result<fs::File, io::Error> {
        let mut file = fs::File::create(path)?;
        file.write_all(&self.bytes)?;
        Ok(file)
    }

    /// Saves the image to an existing file
    /// 
    /// # Example
    /// ```
    /// let meme = Meme::get("minecraft").unwrap();
    /// 
    /// let file = meme.image.save_on_existing("meme.png");
    /// ```
    pub fn save_as_existing(&self, path : &str) -> Result<fs::File, io::Error> {
        let mut file = fs::File::open(path)?;
        file.write_all(&self.bytes)?;
        Ok(file)
    }
}

/// The Meme Struct
#[derive(Debug)]
pub struct Meme {
    pub image : Image,
    pub subreddit : String,
    pub score : u64,
    pub id : String,
    pub self_text : String,
    pub is_nsfw : bool
}

impl Meme {
    pub fn get(topic : &str) -> Result<Meme, Error> {
        
        let bytes = request(&format!("meme/{}",topic).to_string())?;


        let json : serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        
        let bytes = _request(json["data"]["url"].as_str().unwrap())?;

        let image = Image {
            url : Some(json["data"]["url"].as_str().unwrap().to_string()),
            bytes : bytes
        };

        Ok(
            Meme {
                image : image,
                subreddit : json["data"]["subreddit"].as_str().unwrap().to_string(),
                score : json["data"]["score"].as_u64().unwrap(),
                id : json["data"]["id"].as_str().unwrap().to_string(),
                self_text : json["data"]["selftext"].as_str().unwrap().to_string(),
                is_nsfw : json["data"]["is_nsfw"].as_bool().unwrap()
            }
        )
    }
}

/// Get a random meme
/// 
/// # Example
/// ```
/// let meme = get_random_meme().unwrap();
/// ```
pub fn get_random_meme() -> Result<Image, Error> {
    
    let bytes = request("meme")?;

    Ok(
        Image {
            url : None,
            bytes : bytes
        }
    )
}

/// Get an answer from the magic 8ball
/// 
/// # Example
/// ```
/// let answer = eighball().unwrap();
/// ```  
pub fn eightball() -> Result<String, Error> {

    let bytes = request("8ball?simple=True")?;

    Ok(
        String::from_utf8(bytes.as_ref().to_vec()).unwrap()
    )
}

/// Would you rather
/// 
/// # Example
/// ```
/// let answer = would_you_rather().unwrap();
/// ```
pub fn would_you_rather() -> Result< (String , String) , Error> {

    let bytes = request("wyr")?;

    let json : serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    Ok(
        (
            json["data"]["Would you rather"][0].as_str().unwrap().to_string(),
            json["data"]["Would you rather"][1].as_str().unwrap().to_string()
        )
    )
}

/// Get a Truth or Dare
/// 
/// # Example
/// ```
/// let answer = truth_or_dare().unwrap();
/// ```
pub fn truth_or_dare() -> Result< (String, String) , Error> {

    let bytes = request("truthordare")?;

    let json : serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    Ok(
        (
            json["data"]["Truth"].as_str().unwrap().to_string(),
            json["data"]["Dare"].as_str().unwrap().to_string(),
        )
    )
}

/// Get a Truth
/// 
/// # Example
/// ```
/// let answer = truth().unwrap();
/// ```
pub fn truth() -> Result<String , Error> {

    let bytes = request("truth?simple=True")?;

    Ok(
        String::from_utf8(bytes.as_ref().to_vec()).unwrap()
    )
}

/// Get a Dare
/// 
/// # Example
/// ```
/// let answer = dare().unwrap();
/// ```
pub fn dare() -> Result<String , Error> {

    let bytes = request("dare?simple=True")?;

    Ok(
        String::from_utf8(bytes.as_ref().to_vec()).unwrap()
    )
}

/// Get a random fact
/// 
/// # Example
/// ```
/// let fact = fact().unwrap();
/// ```
pub fn fact() -> Result<String, Error> {

    let bytes = request("fact?simple=True")?;

    Ok(
        String::from_utf8(bytes.as_ref().to_vec()).unwrap()
    )

}

/// Roast someone
/// 
/// # Example
/// ```
/// let roast = roast().unwrap();
/// ```
pub fn roast() -> Result<String, Error> {

    let bytes = request("roast?simple=True")?;

    Ok(
        String::from_utf8(bytes.as_ref().to_vec()).unwrap()
    )

}

#[derive(Debug)]
pub struct Trivia {
    pub ques : String,
    pub ans : String
}

impl Trivia {
    /// Get a trivia question with its answer
    /// 
    /// # Example
    /// ```
    /// let trivia = Trivia::get().unwrap();
    /// ```
    pub fn get() -> Result<Trivia, Error> {

        let bytes = request("trivia")?;
    
        let json : serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        Ok(
            Trivia {
                ques : json["data"]["Question"].as_str().unwrap().to_string(),
                ans : json["data"]["Answer"].as_str().unwrap().to_string()
            }
        )
    }

}

/// Get a joke
/// 
/// # Example
/// ```
/// let joke = joke().unwrap();
/// ```
pub fn joke() -> Result<String, Error> {

    let bytes = request("joke?simple=True")?;

    Ok(
        String::from_utf8(bytes.as_ref().to_vec()).unwrap()
    )

}

/// Get a compliment
/// 
/// # Example
/// ```
/// let compliment = compliment().unwrap();
/// ```
pub fn compliment() -> Result<String, Error> {

    let bytes = request("compliment?simple=True")?;

    Ok(
        String::from_utf8(bytes.as_ref().to_vec()).unwrap()
    )

}