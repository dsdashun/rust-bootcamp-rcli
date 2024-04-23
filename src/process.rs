use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(100);
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        // headers.iter() => 使用 headers 的迭代器
        // record.iter() => 使用 record 的迭代器
        // zip => 将两个迭代器合并成一个元组的迭代器 [(header1, record1), (header2, record2), ...]
        // collect::<Value>() => 将元组的迭代器转换成 Value 类型
        let json_record = headers.iter().zip(record.iter()).collect::<Value>();
        println!("{:?}", record);
        ret.push(json_record);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
