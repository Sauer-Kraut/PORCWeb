use regex::Regex;
use serde::Serialize;
use std::{
    collections::HashMap,
    env, error, fs,
    path::PathBuf,
};
use walkdir::WalkDir;

#[derive(Debug, Clone)]
struct SQLFunctionExport {
    sig: FunctionSignature,
    body: String,
    imports: Vec<String>,
    file_path: PathBuf,
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=sql");

    // `(?m)` so `^` matches the start of *any* line, not just the start of
    // the whole file - otherwise `@export` only ever matched if it was the
    // very first byte in the file.
    let export_regex =
        Regex::new(r"(?m)^\s*@export\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(([^()]*)\)").unwrap();

    // Used to discover which other exported functions a given file calls,
    // so we can populate `imports` and later resolve the dependency tree.
    let include_regex = Regex::new(r"@include\s+([A-Za-z_][A-Za-z0-9_]*)\s*\(").unwrap();

    let mut unbuilt: HashMap<String, SQLFunctionExport> = HashMap::new();

    for entry in WalkDir::new("src") {
        let entry = entry.unwrap();

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        if path.extension().and_then(|e| e.to_str()) != Some("sql") {
            continue;
        }

        let contents = fs::read_to_string(path).unwrap();

        let Some(captures) = export_regex.captures(&contents) else {
            // Not every .sql file needs to be an export (it might just be
            // a private helper only ever pulled in via @include).
            continue;
        };

        let func_name = captures[1].trim().to_string();
        let args_str = captures[2].trim();

        let mut arguments = Vec::new();
        if !args_str.is_empty() {
            for arg in args_str.split(',') {
                let parts: Vec<&str> = arg.trim().splitn(2, ':').collect();
                if parts.len() != 2 {
                    panic!(
                        "Faulty SQL function argument '{}' in {}",
                        arg,
                        path.display()
                    );
                }

                let arg_name = parts[0].trim().to_string();
                let arg_type: DefinitionArgumentType = parts[1].try_into().map_err(|e| {
                    panic!("Faulty SQL function definition at {}: {}", path.display(), e)
                }).unwrap();

                arguments.push(FunctionArgument { arg_name, arg_type });
            }
        }

        // Body = the file contents with the `@export ...` directive line
        // stripped out.
        let export_match = captures.get(0).unwrap();
        let body = format!(
            "{}{}",
            &contents[..export_match.start()],
            &contents[export_match.end()..]
        )
        .trim()
        .to_string();

        let mut imports = Vec::new();
        for cap in include_regex.captures_iter(&body) {
            let dep_name = cap[1].to_string();
            if !imports.contains(&dep_name) {
                imports.push(dep_name);
            }
        }

        let export = SQLFunctionExport {
            sig: FunctionSignature {
                name: func_name.clone(),
                arguments,
            },
            body: body.to_owned(),
            imports,
            file_path: path.to_path_buf(),
        };

        if let Some(existing) = unbuilt.insert(func_name.clone(), export) {
            panic!(
                "Duplicate SQL export '{}' found in both {} and {}",
                func_name,
                existing.file_path.display(),
                path.display(),
            );
        }
    }

    // Resolve every export's @include calls into fully inlined SQL bodies.
    let mut build: HashMap<String, SQLFunctionExport> = HashMap::new();
    for (name, function) in unbuilt.iter() {
        let mut visited = Vec::new();
        build_function_tree(function, &mut build, &mut visited, &unbuilt)
            .unwrap_or_else(|e| panic!("Failed to build SQL function '{}': {}", name, e));
    }

    // Write one compiled .sql file per exported function, plus a JSON
    // manifest describing every compiled function (signature + locations).
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let sql_out_dir = manifest_dir.join("build").join("sql");
    fs::create_dir_all(&sql_out_dir).expect("Failed to create build/sql output directory");

    let mut names: Vec<&String> = build.keys().collect();
    names.sort();

    let mut compiled_exports = Vec::with_capacity(names.len());

    for name in names {
        let function = &build[name];
        let out_file = sql_out_dir.join(format!("{}.sql", name));

        fs::write(&out_file, &function.body)
            .unwrap_or_else(|e| panic!("Failed to write SQL output for '{}': {}", name, e));

        compiled_exports.push(CompiledFunctionExport {
            name: function.sig.name.clone(),
            arguments: function
                .sig
                .arguments
                .iter()
                .map(|a| CompiledFunctionArgument {
                    name: a.arg_name.clone(),
                    arg_type: a.arg_type.as_str().to_string(),
                })
                .collect(),
            source_file: function.file_path.display().to_string(),
            output_file: out_file.display().to_string(),
        });
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let json_dest = out_dir.join("sql_functions.json");
    let json = serde_json::to_string_pretty(&compiled_exports)
        .expect("Failed to serialize compiled SQL function registry to JSON");
    fs::write(&json_dest, json).expect("Failed to write sql_functions.json");
}

#[derive(Debug, Clone, Serialize)]
struct CompiledFunctionExport {
    name: String,
    arguments: Vec<CompiledFunctionArgument>,
    source_file: String,
    output_file: String,
}

#[derive(Debug, Clone, Serialize)]
struct CompiledFunctionArgument {
    name: String,
    arg_type: String,
}

#[derive(Debug, Clone)]
pub enum DefinitionArgumentType {
    String,
    Int,
    Float,
    Bool,
    Timestamptz,
    JSONB,
    Null,
}

impl DefinitionArgumentType {
    fn as_str(&self) -> &'static str {
        match self {
            DefinitionArgumentType::String => "string",
            DefinitionArgumentType::Int => "int",
            DefinitionArgumentType::Float => "float",
            DefinitionArgumentType::Bool => "bool",
            DefinitionArgumentType::Timestamptz => "timestamptz",
            DefinitionArgumentType::JSONB => "jsonb",
            DefinitionArgumentType::Null => "null",
        }
    }
}

impl TryFrom<&str> for DefinitionArgumentType {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "string" => Ok(DefinitionArgumentType::String),
            "int" => Ok(DefinitionArgumentType::Int),
            "float" => Ok(DefinitionArgumentType::Float),
            "bool" => Ok(DefinitionArgumentType::Bool),
            "timestamptz" => Ok(DefinitionArgumentType::Timestamptz),
            "jsonb" => Ok(DefinitionArgumentType::JSONB),
            "null" => Ok(DefinitionArgumentType::Null),
            other => Err(format!("Unknown SQL argument type '{}'", other)),
        }
    }
}

#[derive(Debug, Clone)]
struct FunctionArgument {
    arg_name: String,
    arg_type: DefinitionArgumentType,
}

#[derive(Debug, Clone)]
struct FunctionSignature {
    name: String,
    arguments: Vec<FunctionArgument>,
}

fn generate_import(
    query: String,
    imp_query: String,
    imp_sig: FunctionSignature,
) -> Result<String, Box<dyn error::Error>> {
    let import_statement_regex = Regex::new(
        &(r"@include\s*".to_string() + &regex::escape(&imp_sig.name) + r"\s*\(([^()]*)\)"),
    )?;

    let mut out = query.clone();

    // Collect every call to this function first, then replace them all -
    // `captures()` (singular) only ever finds the *first* match, so a
    // function called more than once in the same body would previously
    // only have its first occurrence inlined.
    let calls: Vec<(String, String)> = import_statement_regex
        .captures_iter(&query)
        .map(|captures| {
            let full_call = captures.get(0).unwrap().as_str().to_string();
            let args = captures.get(1).unwrap().as_str().to_string();
            (full_call, args)
        })
        .collect();

    for (call, args_str) in calls {
        let arguments: Vec<String> = if args_str.trim().is_empty() {
            Vec::new()
        } else {
            args_str.split(',').map(|s| s.trim().to_string()).collect()
        };

        if arguments.len() != imp_sig.arguments.len() {
            return Err(format!(
                "Supplied arguments for call '{}' did not match function signature of '{}' (expected {}, got {})",
                call,
                imp_sig.name,
                imp_sig.arguments.len(),
                arguments.len()
            )
            .into());
        }

        let mut populated_import = imp_query.clone();
        for (index, arg) in imp_sig.arguments.iter().enumerate() {
            populated_import =
                populated_import.replace(&format!("${}", arg.arg_name), &arguments[index]);
        }

        out = out.replace(&call, &format!("({})", populated_import));
    }

    Ok(out)
}

fn build_function_tree(
    function: &SQLFunctionExport,
    build: &mut HashMap<String, SQLFunctionExport>,
    visited: &mut Vec<String>,
    unbuilt: &HashMap<String, SQLFunctionExport>,
) -> Result<(), Box<dyn error::Error>> {
    // Already fully resolved (e.g. as someone else's dependency) - reuse it.
    if build.contains_key(&function.sig.name) {
        return Ok(());
    }

    if visited.contains(&function.sig.name) {
        return Err(format!(
            "Circular dependency detected involving SQL function '{}'",
            function.sig.name
        )
        .into());
    }

    visited.push(function.sig.name.clone());

    let mut build_function = function.body.clone();
    for dep in function.imports.iter() {
        if !build.contains_key(dep) {
            let dep_fn = unbuilt.get(dep).ok_or_else(|| {
                format!(
                    "Missing SQL function export for dependency '{}' (required by '{}')",
                    dep, function.sig.name
                )
            })?;
            build_function_tree(dep_fn, build, visited, unbuilt)?;
        }
        let imp = build
            .get(dep)
            .expect("dependency should have been built by now");
        build_function = generate_import(build_function, imp.body.clone(), imp.sig.clone())?;
    }

    // Pop before returning - this stack represents the *current* DFS path,
    // not the set of everything ever visited, otherwise two unrelated
    // functions sharing a dependency would falsely trigger a "cycle".
    visited.pop();

    build.insert(
        function.sig.name.clone(),
        SQLFunctionExport {
            sig: function.sig.clone(),
            body: build_function,
            imports: function.imports.clone(),
            file_path: function.file_path.clone(),
        },
    );

    Ok(())
}