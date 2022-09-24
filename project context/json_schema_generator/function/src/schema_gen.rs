#![allow(unused_variables)]
#![allow(unused_imports)]

//General
use std::fs::File;

//Async
use async_trait::async_trait;
use tokio;

//Serverless
use interface_subsystems::errors::{SamplingError, ServiceIOError};
use interface_subsystems::interfaces::ServerlessCommand; //abstract for serverless command
use util_cmd::net::faas::send; //Send command call request to FaaS

//JSON Schema Generator
use serde_json::{ Value, json, from_str, from_reader, to_string_pretty };
use infers_jsonschema::infer;                   // Infer JSON Schemas
use assert_json_diff::assert_json_eq_no_panic; // Compare JSON Schemas


  //////////////
 //Serverless//
//////////////
pub struct SchemaServerlessTest {
    pub name: String,
}
#[async_trait]
impl ServerlessCommand for SchemaServerlessTest {
    fn new() -> Self {
        Self {
            name: String::from("schema_gen_TEST"),
        }
    }

    async fn call_serverless(&self, data: Box<Value>) -> Result<Value, ServiceIOError> {
        let data_json = *data;
        let svrls_call = send(&self.name, &data_json)
            .await
            .expect("We got the response from the serverless function.");
        Ok(svrls_call)
    }
}

pub struct SchemaServerless {
    pub name: String,
}

#[async_trait]
impl ServerlessCommand for SchemaServerless {
    fn new() -> Self {
        Self {
            name: String::from("schema_gen"),
        }
    }

    async fn call_serverless(&self, data: Box<Value>) -> Result<Value, ServiceIOError> {
        let data_json = *data;
        let svrls_call = send(&self.name, &data_json)
            .await
            .expect("We got the response from the serverless function.");
        Ok(svrls_call)
    }
}

   /////////////////////////
  //JSON Schema Generator//
 /////////////////////////
///Recursive complete map of keys in schema
fn deep_keys(value: &Value, 
             current_path: Vec<String>, 
             output: &mut Vec<Vec<String>>) {
    if current_path.len() > 0 {
        output.push(current_path.clone());
    }

    match value {
        Value::Object(map) => {
            for (k, v) in map {
                let mut new_path = current_path.clone();
                new_path.push(k.to_owned());
                deep_keys(v, new_path, output);
            }
        },
        Value::Array(array) => {
            for (i, v) in array.iter().enumerate() {
                let mut new_path = current_path.clone();
                new_path.push(i.to_string().to_owned());
                deep_keys(v, new_path, output);
            }
        },
        _ => ()
    }
}    
///Expected JSON input types
#[derive(Debug)]
pub enum DocumentCollection {
    File_(File), //todo untested
    FilePath(String), 
    JsonString(String),
    JsonValue(Value) 
}
///User choice for schema return format
#[derive(Debug)]
pub enum JSONSchemaReturnFormat {
    JsonString(String),
    JsonValue(Value) 
}
///Convert a [std:fs:file, file path string or json String] -> serde_json::Value
pub fn load_document_array(document_array: DocumentCollection) -> Value { 
    let json_records: Value = match document_array {
        DocumentCollection::File_     (file)        => from_reader(file).unwrap(),
        DocumentCollection::FilePath  (file_path)   => from_reader(File::open(file_path).unwrap()).unwrap(),
        DocumentCollection::JsonString(json_string) => from_str(&json_string as &str).unwrap(),
        DocumentCollection::JsonValue (json_value)  => json_value };
    return json_records;
}
///Array of Documents -> infer composite schema + required properties
pub fn generate_master_schema(
    document_array: DocumentCollection, 
    return_format : &str) 
 -> JSONSchemaReturnFormat {
    /* 
    param: document_array: DocumentCollection = any of these:
        DocumentCollection::File_(std::fs::File)
        DocumentCollection::FilePath(String)
        DocumentCollection::JsonString(String)
        DocumentCollection::JsonValue(serde_json::Value)

    param: return_format: &str 
        options are JsonString: String and JsonValue: serde_json::Value
        these correspond to the JSONSchemaReturnFormat::* enum values
    */

    //Load document collection as serde Value 
    let json_records: Value = load_document_array(document_array);
    // Take first json document as default schema
    let mut new_schema   : Value = infer(&json_records[0]);
    let mut master_schema: Value = new_schema.clone();
    // Deconstruct schemas to remove the default "required" array
    if let Value::Object(o) = &mut master_schema { o.remove("required"); }
    if let Value::Object(o) = &mut new_schema    { o.remove("required"); }
    // let them input custom title? or standardized?

    let mut diff_msg           :         String;
    let mut all_diff_paths     :     Vec<String>  = vec!();
    let mut all_diff_paths_vecs: Vec<Vec<String>> = vec!();

    // Deconstruct json records as Array or HashMap
    // If HashMap, take the keys & values of the top level and make a Vec of Values 
    let _json_records = match &json_records {
        Value::Array(a)  => a.clone(),
        Value::Object(o) => o.into_iter()
                             .map( |(key, val)| json![{key: val}] )
                             .collect(),
        _ => vec![] };

    // Iterate through the list of documents in the loaded json file -> infer schema
    for record in _json_records {
        // infer schema for a record
        new_schema = infer(&record);
        //println!("{}", &new_schema);
        
        // Diff json objects/schemas
        let json_diff_response = assert_json_eq_no_panic(&new_schema, &master_schema);
        diff_msg = match json_diff_response {
            Ok(_) => "".to_string(),
            Err(missing_data) => missing_data }; 

        // if identical jsons => skip
        if diff_msg.len() == 0 { continue; } 

        println!("\n[Next Record Schema Diffs]:"); 
        // todo fix that it prints even when 0 diffs are printed

        // Split Message into parsable strings, each is a missing field
            // json atom at path ".required" is missing from rhs\n
            // json atom at path ".properties.name.format" is missing from lhs
        let temp_diff_strings: Vec<String> = diff_msg.split("\n").map(|s| s.to_string()).collect();

        // Filter out the 4 elements after a diff message containing 'are not equal'
            // json atoms at path ".properties.book.type" are not equal:
            // lhs:
            //     "integer"
            // rhs:
            //     "string"
        let mut diff_strings : Vec<String> = vec![];
        for diff in temp_diff_strings {
            if diff == "" { continue; }
            if diff.contains("required") { continue; } // Avoid adding the inferred required fields, we build it later
            if diff.contains("are not equal") || diff.contains("missing from lhs")  || diff.contains("missing from rhs") { // skip next 4 elements
                if !diff_strings.contains(&diff) { 
                    println!("{}", &diff); // Print Diff
                    diff_strings.push(diff); 
                }
            }
        }

        // Take each Diff and integrate it into master schema if relevant
        for diff_string in diff_strings {
            if diff_string.len() == 0 { continue; }
            // if missing base properties level skip
            if diff_string.contains("\".properties\" is missing from lhs") { continue; } 

            // Rip path from diff message
            let diff_split:       Vec<String> = diff_string .split("\"").map(|s| s.to_string()).collect();
            // Rip keys from path
            let missing_path_vec: Vec<String> = diff_split[1].split(".").map(|s| s.to_string()).collect();
            let missing_path:         String  = missing_path_vec.join("/");

            // if inferred record schema is missing a field present in the Master Schema log it as not required
            if diff_string.contains("missing from lhs") {
                //eg. json atom at path ".properties.name.type" is missing from lhs
                if !all_diff_paths.contains(&missing_path) { 
                    //println!("{} was added to the optional property list", &missing_path);
                    all_diff_paths     .push(missing_path); // Store all unique missing paths 
                    all_diff_paths_vecs.push(missing_path_vec); // to determine non-required properties 
                }
            } // else if field missing from the Master Schema (Right Hand Side)
            else if diff_string.contains("missing from rhs") {

                //todo if missing from RHS then dont need to reset that uniqueness every loop, only equality
                let path = missing_path_vec[0..missing_path_vec.len()-1].join("/");
                //println!("{}", &path);
                let mut missing_value: Value = new_schema.pointer(&missing_path as &str).unwrap().clone();

                // Use Pointer notation to edit/access hashmap json object
                // let hashmap = {a: {b: {c: 9}}}
                // hashmap.pointer("/a/b/c") == 9
                // here we use .pointer_mut() => Options<&mut Value> => Value => Object, Array or String
                match master_schema.pointer_mut(&path as &str) {
                    None => (),
                    Some(mut master_value) => {
                        //if master_value is array
                        if let Value::Array(master_array) = &mut master_value { 
                            // if new val is array, extend master array
                            if let Value::Array(new_array) = missing_value {
                                master_array.extend(new_array);
                            } // else if new val is String, push to master array
                            else if let Value::String(new) = missing_value {
                                master_array.push(json![new]); }
                        } // else if master_value is hashmap
                        else if let Value::Object(master_map) = &mut master_value { 
                            // insert missing key-value into master_map
                            match missing_path_vec.last() { 
                                None => (),
                                Some(last_key) => {
                                    match master_map.insert(last_key.to_string(), missing_value) {
                                        None => (),
                                        Some(new_value) => println!("Added {{{}: {}}} to {}", 
                                                                     last_key, &new_value, &path) }}}
                        } else if master_value.is_string() {
                            //if new val is String too, replace master_value with array of all
                            if let Value::String(new_value) = &mut missing_value { 
                                *master_value = json![vec![master_value.as_str().unwrap(), 
                                                          &new_value as &str]]; 
                            } else if let Value::Array(new_array) = &mut missing_value { 
                                new_array.push(master_value.clone());
                                *master_value = json![new_array]; 
                            }
                        }
                    }
                }
            } // else if field present in both schemas but non-identical
            else if diff_string.contains("are not equal") {
                // let mut valid_path_vec = missing_path_vec.clone();
                // let mut i = valid_path_vec.len()-1;
                // while i > 0 {
                //     let path = &valid_path_vec.join("/") as &str;
                //     i = i - 1;
                //     match master_schema.pointer(path) {
                //         None => valid_path_vec.truncate(path.len()-1),
                //         Some(_) => break
                //     }
                // }
                // let valid_path  :     String  =  valid_path_vec.join("/");
                let path = missing_path_vec.join("/");
                //println!("{}", &path);

                // let missing_keys: Vec<String> =  missing_path_vec.get(
                //     valid_path_vec.len()..missing_path_vec.len()
                // ).unwrap().to_vec();
                let mut missing_value: Value = new_schema.pointer(&missing_path as &str).unwrap().clone();

                // Use Pointer notation to edit/access hashmap json object
                // let hashmap = {a: {b: {c: 9}}}
                // hashmap.pointer("/a/b/c") == 9  
                // here we use .pointer_mut() => Options<&mut Value> => Value => Object, Array or String
                match master_schema.pointer_mut(&path as &str) { 
                    None => (),
                    Some(mut master_value) => {
                        //if master_value is array
                        if let Value::Array(a) = &mut master_value { 
                            if !a.contains(&missing_value){
                                a.push(missing_value)
                            }
                        } else if let Value::Object(o) = &mut master_value { 
                            match missing_path_vec.last() { //If master is HashMap -> Insert Missing Key-Value
                                None => (),
                                Some(last_key) => {
                                    match o.insert(last_key.to_string(), missing_value) {
                                        None => (),
                                        Some(new_value) => println!("Added {}: {} to {}", 
                                                                     last_key, &new_value, &path)}}}
                        } else if master_value.is_string() {
                            //Replace String with Array due to multiple values now found for this
                            if let Value::String(new_value) = &mut missing_value { 
                                *master_value = json![vec![master_value.as_str().unwrap(), 
                                                          &new_value as &str]]; 
                            } else if let Value::Array(new_array) = &mut missing_value { 
                                new_array.push(master_value.clone()); //push master_value to new array
                                *master_value = json![new_array]; }}} //update master_value from String to Array
                }
            }
        }
    }

    println!("\n[Creating Required Properties List]");
    let mut required_props = master_schema.clone();
    let all_diff_paths_vecs_ = all_diff_paths_vecs.clone();
    for _path_vec in all_diff_paths_vecs_ {
        let mut path_vec = _path_vec.clone();
        while path_vec.contains(&"properties".to_string()) {
            let index = match path_vec.iter().rposition(|x| x == "properties")  //signifying deepest nesting layer
                          { Some(index) => index, None => 0 };
            if index == 0 { continue; }

            let slice_start: usize = 0; // unsure if ever not 0
            let json_path  : &str  = &path_vec[slice_start..index+1].join("/"); 

            match required_props.pointer_mut(&json_path as &str) {
                Some(mut properties_obj) => {
                    if let Value::Object(properties) = &mut properties_obj { 
                        let property_key: &str = &path_vec[index+1];
                        if properties.contains_key(property_key) {
                            match properties.remove(property_key) {
                                Some(_) => {
                                        println!("Removed \"{}\" from the required properties list as it was missing in at least 1 record", property_key); },
                                None => println!("Failed to remove {} from the Required properties list", property_key)
                            }
                        }
                    }
                },
                None => ()
            }          
            // truncate away the lastmost layer of "properties"
            path_vec.truncate(index);
        }
    }


    let mut deep_keys_all_paths = vec![vec![]];
    let current_path = vec![];
    deep_keys(&required_props, current_path, &mut deep_keys_all_paths);
    //println!("{:?}", &deep_keys_all_paths);
    // Could drop a bunch of useless paths by editing deepkeys 
    // to only push a Vec when at end of a branch
    
    // Add required property to the schema if prop was present in all of the docs
    for _path_vec in deep_keys_all_paths {
        let mut path_vec = _path_vec.clone();
        path_vec.insert(0, "".to_string());
        while path_vec.contains(&"properties".to_string()) {
            let index = match path_vec.iter().rposition(|x| x == "properties")  //signifying deepest nesting layer
                          { Some(index) => index, None => 0 };
            if index == 0 { continue; }

            let slice_start    : usize  = 0; // unsure if ever not 0
            let parent_path    : &str = &path_vec[slice_start..index  ].join("/"); 
            let properties_path: &str = &path_vec[slice_start..index+1].join("/"); 
            if !properties_path.contains("/") { continue; }

            let required_keys: Vec<Value> = match required_props.pointer(properties_path) {
                None => vec![],
                Some(properties_obj) => properties_obj.as_object().unwrap().clone().into_iter()
                                                      .map(|(key, _val)| serde_json::to_value(key).unwrap())
                                                      .collect() };
            match master_schema.pointer_mut(&parent_path) {
                None => (),
                Some(mut parent_obj) => {
                    if let Value::Object(o) = &mut parent_obj { 
                        if !o.contains_key("required") { o.insert("required".to_string(), Value::Array(Vec::new())); }
                        if let Value::Array(a) = &mut o["required"] { 
                            for key in required_keys.clone() {
                                if !a.contains(&key) {
                                    println!("Added {} to the \"{}\" required properties list as it is ubiquitous in the inputted records", 
                                             &key, properties_path); 
                                    a.push(key);
                                }
                            }                           
                        }
                    }
                }
            }
            // truncate away the lastmost layer of "properties"
            path_vec.truncate(index);
        }
    }

    // After everything return the master_schema
    return match return_format {
        "JsonValue"  => JSONSchemaReturnFormat::JsonValue (master_schema),
        "JsonString" => JSONSchemaReturnFormat::JsonString(master_schema.to_string()),
                   _ => JSONSchemaReturnFormat::JsonString("{}".to_string())
    }
}

///return example JSON document collection as serde_json::Value
pub fn json_gen() -> Result<Box<Value>, SamplingError> {
    let records: Box<Value> = Box::new( 
        serde_json::from_reader(
            std::fs::File::open("./src/example_data.json").unwrap()).unwrap());
    Ok(records)
}


#[cfg(test)]
mod test_super {
    use super::*;

    #[tokio::test]
    async fn test_create_schema_serverless_basic() {
        let srvls_schema = SchemaServerlessTest::new();
        let json_response = json_gen().expect("Was able to generate a basic json file.");
        let srvls_res = srvls_schema
            .call_serverless(json_response)
            .await
            .expect("Was able to send faas command.");
        println!()
        // assert_eq!(generate_schema(json_response), true);
    }
    // tokio::spawn(async move {println!("test1");});



    #[tokio::test]
    async fn test_create_schema_serverless_full() {
        let _master_schema: JSONSchemaReturnFormat = generate_master_schema(
            DocumentCollection::FilePath("./src/example_data.json".to_string()),
            &"JsonString" //schema_gen::JSONSchemaReturnFormat
        );



        let mut master_schema = json![{}]; //init empty
        //Deconstruct enum to serde::Value or string
        if let JSONSchemaReturnFormat::JsonValue(schema) = _master_schema {
            master_schema = schema; 
        }//else convert string to serde::Value
        else if let JSONSchemaReturnFormat::JsonString(schema_str) = _master_schema {
            match from_str(&schema_str) {
                Ok(schema) => master_schema = schema,
                Err(e) => println!("[Error converting master schema from string to serde::Value]:\n    {}", e)}} 
        println!("\n[Generated Master Schema]\n{}", to_string_pretty(&master_schema).unwrap());






        // Validate all documents in ./src/example_data.json against the master_schema
        // All should pass validation because the master schema was made from them
        //
        // Compile Master Schema into a Valico lib validatable json schema
        let mut scope = valico::json_schema::Scope::new();
        let _master_schema = scope.compile_and_return(master_schema.clone(), false).unwrap();

        // Load example json data
        let example_data: Value = *json_gen().unwrap();
        
        // Validate each record against the master schema
        let mut failure_counter = 0;
        for record in example_data.as_array().unwrap() {
            let obj = _master_schema.validate(&record);
            if !obj.is_valid() {
                println!("\n\n[Record with Validation Error]: {}\n[Details]: {}", 
                    to_string_pretty(&record).unwrap(), 
                    to_string_pretty(&obj).unwrap());
                failure_counter = failure_counter + 1;
            }
        } // More than 0 Fails
        if failure_counter > 0 { 
            panic!("{} records failed validation against the master schema\n(which was created to factor in every record and expect 0 failures)",
                    failure_counter);
        } else { println!("\nAll Records Passed Validation!"); }
    }
}

// 3. Replace the prints with logs 
//     //https://github.com/kivo360/monorepo/blob/9a7a6ea30ec7b3833950e1a1b3cc8b5b075e14a6/src/rust/service_bodhi/src/main.rs