use crate::rest::Data;
use bincode;
use std::error::Error;

pub fn en(data: Data) -> Result<Vec<u8>, Box<dyn Error>> {
    return Ok(bincode::serialize(&data)?);
}

pub fn de(data: Vec<u8>) -> Result<Data, Box<dyn Error>> {
    return Ok(bincode::deserialize(&data[..])?);
}
