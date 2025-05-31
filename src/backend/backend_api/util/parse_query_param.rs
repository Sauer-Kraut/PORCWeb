use std::str::FromStr;

// its awful that this has to exist, but I dont think there really is a way around it except not using queries
pub fn parse_query_param_to_vec<T>(parameter: String) -> Result<Vec<T>, Box<dyn std::error::Error>>
where
    T: FromStr
{
    let res = parameter
        .split(',')
        .map(|v| v.trim().chars().filter(|c| *c != '"').collect::<String>().parse::<T>().map_err(|_| "parsing error".to_string()))
        .collect::<Result<Vec<T>, String>>();

    match res {
        Ok(v) => Ok(v),
        Err(e) => Err(e.into()),
    }
}