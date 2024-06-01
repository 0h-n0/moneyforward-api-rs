use reqwest;


pub fn parse_response<T>(text: String, status: reqwest::StatusCode) -> Result<(T, reqwest::StatusCode), Box<dyn std::error::Error>>
    where T: serde::de::DeserializeOwned
{
    match status {
        reqwest::StatusCode::OK => {
            match serde_json::from_str(&text) {
                Ok(obj) => Ok((obj, status)),
                Err(msg) => Err(format!("msg=>{}, text={}", msg, &text).into()),
            }
        }
        _ => {
            Err(format!("status=>{}, msg=>{}", status, text).into())
        }
    }
}

