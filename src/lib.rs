pub mod core;

pub use self::core::*;

/// The Error enum
#[derive(Debug)]
pub enum Error {
    ValidationError(String),
    HTTPException(String)
}

/// Makes a GET request to an endpoint of the api 
/// 
/// # Example
/// ```
/// let bytes = request("meme/minecraft").unwrap();
/// ```
pub fn request(endpoint : &str) -> Result<bytes::Bytes, Error> {
    
    let resp = reqwest::blocking::get(format!("https://api.dhravya.me/{}", endpoint)).unwrap();

    let status : u16 = resp.status().as_u16();

    if status == 200 {
        let bytes : bytes::Bytes = resp.bytes().unwrap();
        return Ok(bytes);
    } else {
        if status == 500 {
            return Err(Error::HTTPException("Internal Server Error".to_string()));
        } else {
            return Err(Error::HTTPException("Unknown Error".to_string()));
        }
    }
}

/// Makes a GET request to a give url 
/// 
/// # Example
/// ```
/// let bytes = _request("https://rust-lan.org").unwrap();
/// ```
pub fn _request(url : &str) -> Result<bytes::Bytes, Error> {
    
    let resp = reqwest::blocking::get(url).unwrap();

    let status : u16 = resp.status().as_u16();

    if status == 200 {
        let bytes : bytes::Bytes = resp.bytes().unwrap();
        return Ok(bytes);
    } else {
        if status == 500 {
            return Err(Error::HTTPException("Internal Server Error".to_string()));
        } else {
            return Err(Error::HTTPException("Unknown Error".to_string()));
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
