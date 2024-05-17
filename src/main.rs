// Copyright (c) 2024 Hemashushu <hippospark@gmail.com>, All rights reserved.
//
// This Source Code Form is subject to the terms of
// the Mozilla Public License version 2.0 and additional exceptions,
// more details in file LICENSE, LICENSE.additional and CONTRIBUTING.

use std::{error::Error, fs::File, io::Write, process};

use ason::{format, AsonNode, VariantItem};
use serde_derive::Deserialize;

const SRC_FILE_PATH: &str = "./data/x86-encoding-csv/x86.csv";
const DST_FILE_PATH: &str = "./data/x86_64.ason";

fn main() {
    if let Err(err) = convert_csv_to_ason() {
        println!("Converting failed: {}", err);
        process::exit(1);
    } else {
        println!("File saved to {}", DST_FILE_PATH);
    }
}

#[derive(Deserialize)]
struct EncodingRecord {
    #[serde(rename = "Instruction")]
    instruction: String,
    #[serde(rename = "Opcode")]
    opcode: String,
    #[serde(rename = "Valid 64-bit")]
    valid_64: String,
    #[allow(dead_code)]
    #[serde(rename = "Valid 32-bit")]
    valid_32: String,
    #[allow(dead_code)]
    #[serde(rename = "Valid 16-bit")]
    valid_16: String,
    #[allow(dead_code)]
    #[serde(rename = "Feature Flags")]
    feature_flag: String,
    #[serde(rename = "Operand 1")]
    operand1: String,
    #[serde(rename = "Operand 2")]
    operand2: String,
    #[serde(rename = "Operand 3")]
    operand3: String,
    #[serde(rename = "Operand 4")]
    operand4: String,
    #[allow(dead_code)]
    #[serde(rename = "Tuple Type")]
    tuple_type: String,
    #[allow(dead_code)]
    #[serde(rename = "Description")]
    description: String,
}

/**
 * Convert the original table into a simplified one.
 *
 * Only the following fildes are needed:
 * - 'instruction': the syntax of instruction
 * - 'opcode': the description of encoding
 * - 'operand1' ... 'operand4': the detail of operands.
 */
fn convert_csv_to_ason() -> Result<(), Box<dyn Error>> {
    let mut dest_records = vec![];

    let source_file = File::open(SRC_FILE_PATH)?;
    let mut reader = csv::Reader::from_reader(source_file);
    for source_record_result in reader.deserialize::<EncodingRecord>() {
        let source_record = source_record_result?;
        if source_record.valid_64 == "Valid" {
            let dest_record = AsonNode::Tuple(vec![
                AsonNode::String_(source_record.instruction),
                AsonNode::String_(source_record.opcode),
                convert_na_to_option(&source_record.operand1),
                convert_na_to_option(&source_record.operand2),
                convert_na_to_option(&source_record.operand3),
                convert_na_to_option(&source_record.operand4),
            ]);
            dest_records.push(dest_record);
        }
    }

    let record_array = AsonNode::Array(dest_records);
    let ason_text = format(&record_array);

    let mut dest_file = File::create(DST_FILE_PATH)?;
    dest_file.write_all(ason_text.as_bytes())?;

    Ok(())
}

fn convert_na_to_option(s: &str) -> AsonNode {
    let variant_item = if s == "NA" || s.is_empty() {
        VariantItem {
            name: "Option::None".to_owned(),
            value: None,
        }
    } else {
        VariantItem {
            name: "Option::Some".to_owned(),
            value: Some(Box::new(AsonNode::String_(s.to_owned()))),
        }
    };

    AsonNode::Variant(variant_item)
}
