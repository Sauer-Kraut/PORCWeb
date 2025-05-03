use std::{fmt, fs::File, io::Read};
use chrono::{DateTime, Utc};

pub enum ArgumentType {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Timestamptz(DateTime<Utc>),
    JSONB(serde_json::Value),
    Null,
}

// I tried to just properly bind variables, but it wouldnt work so here is my crappy alternative that wont allow users to input ' since that would inject sql code and break the query. 
// I will fix this later, but for now this is a good enough solution.      ------- Copilot suggested I add this second line of the comment, and I really appreciate the optimism, but that shit wont happen

pub fn build_query(query_file_path: &str, args: Vec<ArgumentType>) -> Result<String, QueryBuildError> {
    let mut file = match File::open(query_file_path) {
        Ok(file) => file,
        Err(e) => return Err(QueryBuildError::FileReadError(e)),
    };

    let mut query = String::new();

    match file.read_to_string(&mut query) {
        Ok(_) => (),
        Err(e) => return Err(QueryBuildError::FileReadError(e)),
    };

    for (index, arg) in args.iter().enumerate() {

        let argument_key = format!("(${})", index + 1);
        let argument_value = match arg {
            ArgumentType::String(val) => format!("'{}'", val.replace("'", "")),
            ArgumentType::Int(val) => val.to_string(),
            ArgumentType::Float(val) => val.to_string(),
            ArgumentType::Bool(val) => val.to_string(),
            ArgumentType::Timestamptz(val) => format!("'{}'::timestamptz", val.to_string()), // Didnt test this yet, but should work  (::timestamptz doubling shouldnt cause any issues)
            ArgumentType::JSONB(val) => format!("'{}'::jsonb", val.to_string()), // same as above with doubling
            ArgumentType::Null => "null".to_owned()
        };
        
        let mut filled_query = String::new();
        let parts = query.split(&argument_key).collect::<Vec<&str>>();
        if parts.len() == 1 {
            return Err(QueryBuildError::ArgumentError(format!("No placeholder found for argument: {}", argument_key)));
        }
        
        for (index, part) in parts.iter().enumerate() {
            if index != 0 {
                filled_query.push_str(&format!("{} ", &argument_value));
            }
            filled_query.push_str(part);
        }

        query = filled_query;
    }

    Ok(query)
}




#[derive(Debug)]
pub enum QueryBuildError {
    FileReadError(std::io::Error),
    ArgumentError(String),
}

impl fmt::Display for QueryBuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryBuildError::FileReadError(err) => write!(f, "File read error: {}", err),
            QueryBuildError::ArgumentError(err) => write!(f, "Argument error: {}", err),
        }
    }
}

impl std::error::Error for QueryBuildError{}