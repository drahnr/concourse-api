use indexmap::{IndexSet};
use located_yaml::*;
use voca_rs::case::snake_case;
use std::path::PathBuf;
use itertools::Itertools;

include!{"src/types.rs"}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Call {
    /// The canonical _name_ of the API call
    name: String,
    /// The format string, with named placeholders
    fmt: String,
    /// The named placeholders in order
    arguments: IndexSet<String>,
    /// The http method to use
    method: Method, 
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name_snake_case = snake_case(&self.name);
        let args_w_types = String::from_iter(Itertools::intersperse(self.arguments.iter().map(|arg| format!("{arg}: impl ::std::fmt::Display")), ",".to_owned()));
        let args = String::from_iter(Itertools::intersperse(self.arguments.iter().map(|arg| format!("{arg}={arg}")),",".to_owned()));
        let Self {
            ref name,
            ref fmt,
            method,
            ..
        } = self;
        writeln!(f, r##"
/// {name} Councourse CI API call
pub fn {name_snake_case} ({args_w_types}) -> (String, Method) {{
    (format!("{fmt}", {args}), Method::{method})
}}
"##)
    }
}

fn convert_colons_to_named_placeholders<'a>(s: &'a str) -> (String, IndexSet<String>) {
    let mut api_named_fmt_args = IndexSet::new();
    if s.is_empty() || !s.contains(':') {
        return (s.into(), api_named_fmt_args)
    }
    let api_fmt_str = String::from_iter(Itertools::intersperse(s.split('/').map(|element| {
        if let Some(arg) = element.strip_prefix(':') {
            api_named_fmt_args.insert(arg.to_owned());
            format!("{{{arg}}}")
        } else {
            element.into()
        }
    }),"/".to_owned()));
    (api_fmt_str, api_named_fmt_args)
}

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    
    let yaml_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("api.yaml");
    
    let yaml = std::fs::read_to_string(yaml_path)?;
    let yaml = YamlLoader::load_from_str(&yaml)?;
    let mut raws = yaml.docs;
    assert_eq!(raws.len(), 1);
    let raw = raws.pop().expect("Must contain at least one document");
    
    if let Some(_second) = raws.pop() {
        unreachable!("Contains exactly one document, but we found another..");
    }

    let mut gen = String::with_capacity(4096);
    
    gen.push_str("use crate::Method;\n");
    
    let api_calls_raw = match raw.yaml {
        YamlElt::Array(array) => array,
        _ => unreachable!("The root element must be an array. qed"),
    };
    
    for ref api_raw in api_calls_raw {
        let method = api_raw.get_string_key("Method").unwrap().get_string().unwrap();
        let method = Method::from_str(method.as_str()).unwrap();
        let (fmt, arguments) = convert_colons_to_named_placeholders(api_raw.get_string_key("Path").unwrap().get_string().unwrap().as_str());      
        let name = api_raw.get_string_key("Name").unwrap().get_string().unwrap();
        let call = Call {
            name,
            fmt,
            arguments,
            method
        };
        
        gen.push_str(call.to_string().as_str());
        gen.push_str("\n");
    }
    
    let dest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("Always set OUT_DIR by cargo. qed")).join("src/api.rs");
    std::fs::write(dest, gen)?;
    Ok(())
}
