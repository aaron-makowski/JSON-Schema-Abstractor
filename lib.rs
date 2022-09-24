pub mod schema_gen;
use schema_gen::{ generate_master_schema, JSONSchemaReturnFormat, DocumentCollection };
use serde_json::{ to_string_pretty };

/// FaaS Func recieves request input as String and returns a JSON schema as String
/// handle() is a FaaS required function within function/src/lib.rs
pub fn handle(request: String) -> String {
    // json records as string go in
    // master schema is returned in format of choice
    let _master_schema: JSONSchemaReturnFormat = generate_master_schema(
        DocumentCollection::JsonString(request),
        &"JsonString" //JSONSchemaReturnFormat::JsonString(String)
    );
    // deconstruct returned schema from return format enum wrapper
    if let JSONSchemaReturnFormat::JsonString(schema_str) = _master_schema {
        return format!("Master Schema Generated from JSON Records Successfully through calling FaaS Command\n{}", 
                                                                         to_string_pretty(&schema_str).unwrap());
    } else { return format!("Error generating schema\n{:?}", &_master_schema); }
}




// let record: Value = from_str(r#"
// {
//     "book": "DONG",
//     "author": "powell",
//     "case": "The Master, whose personal name was Liang-chieh, was a member of the Yu family of Kuei-chi. Once, as a child, when reading the Heart Sutra with his tutor, he came to the line, \"There is no eye, no ear, no nose, no tongue, no body, no mind.\" He immediately felt his face with his hand, then said to his tutor, \"I have eyes, ears, a nose, a tongue, and so on; why does the sutra say they don't exist?\"\nThis took the tutor by surprise, and, recognizing Tung-shan's uniqueness, he said, \"I am not capable of being your teacher.\"\nFrom there the Master went to Wu-hsieh Mountain, where, after making obeisance to Ch'an Master Mo, he took the robe and shaved his head. When he was twenty-one he went to Sung Mountain and took the Complete Precepts.\n",
//     "id": 1,
//     "name": "1",
//     "case_number": "1",
//     "case_number": "1"
// }
// "#).unwrap();
// let record2: Value = from_str(r#"
// {
//     "book": "DONG",
//     "author": "powell",
//     "case": "The Master, whose personal name was Liang-chieh, was a member of the Yu family of Kuei-chi. Once, as a child, when reading the Heart Sutra with his tutor, he came to the line, \"There is no eye, no ear, no nose, no tongue, no body, no mind.\" He immediately felt his face with his hand, then said to his tutor, \"I have eyes, ears, a nose, a tongue, and so on; why does the sutra say they don't exist?\"\nThis took the tutor by surprise, and, recognizing Tung-shan's uniqueness, he said, \"I am not capable of being your teacher.\"\nFrom there the Master went to Wu-hsieh Mountain, where, after making obeisance to Ch'an Master Mo, he took the robe and shaved his head. When he was twenty-one he went to Sung Mountain and took the Complete Precepts.\n",
//     "id": 1,
//     "name": "1",
//     "nested_test": {"t":"test"},
//     "case_number": "1"
// }
// "#).unwrap();
// let record3: Value = from_str(r#"{}"#).unwrap();
// // todo Test using nested json, double nested, arbitrary nested adding multiple params

// let record_validation_obj = _master_schema.validate(&record2);
// println!("\n[Test Validity]: {}",                  &record_validation_obj.is_valid());
// println!(  "[Test Details]: {}" , to_string_pretty(&record_validation_obj).unwrap());