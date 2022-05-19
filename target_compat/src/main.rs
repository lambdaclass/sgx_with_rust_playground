use std::error::Error;

fn is_sgx_compatible(crate_identifier: &str) -> bool {

    let output = std::process::Command::new("cargo")
                         .arg("+nightly")
                         .arg("check")
                         .arg("--package")
                         .arg(crate_identifier)
                         .arg("--target")
                         .arg("x86_64-fortanix-unknown-sgx")
                         .status()
                         .expect("failed to execute process");
   output.success() 
}

// We should get a json object containing the crate name, the version and check for its
// dependencies
fn compatible_deps(dep_tree: &json::JsonValue) -> Result<Vec<String>, json::Error> {
    match dep_tree {
        json::JsonValue::Null => Ok(Vec::new()),
        json::JsonValue::Object(o) => {
            let crate_name = &o["name"];
            let crate_version = &o["version"];
            let crate_ident = format!("{}:{}", crate_name, crate_version);
            println!("{}:{}", crate_name, crate_version);

            // If it is sgx compatible then its dependencies will be as well.
            // Otherwise, see recursively which of those are compatible
            if is_sgx_compatible(&crate_ident) {
                Ok(vec![crate_ident])
            } else {
                let deps = &dep_tree["dependencies"];
                if let json::JsonValue::Array(v) = deps {
                    Ok(
                        //v.iter().flat_map(|dep| compatible_deps(dep).unwrap()).collect()
                        vec![]
                    )
                } else {
                    Err(json::Error::WrongType("AHHHHHHH!".to_string()))
                }
            }
            
            
        }
        _ => Err(json::Error::WrongType("AHHHHHHH!".to_string()))
    }    
}

fn main() -> Result<(), Box<dyn Error>> {
    let metadata = json::parse(&std::fs::read_to_string("metadata.json")?)?;

    let dep_tree = &metadata["packages"];
    if let json::JsonValue::Array(v) = dep_tree {

        let all_deps : std::collections::HashSet<String> = std::collections::HashSet::from_iter(v.iter().map(|dep| {
            if let json::JsonValue::Object(o) = dep {
                let crate_name = &o["name"];
                let crate_version = &o["version"];
                format!("{}:{}", crate_name, crate_version).to_string()
            } else {
                "".to_string()
            }
        }));
        let compatible : std::collections::HashSet<String> = std::collections::HashSet::from_iter(v.iter().flat_map(|dep| compatible_deps(dep).unwrap()));
        let incompatible : std::collections::HashSet<String> = all_deps.difference(&compatible).cloned().collect();
        println!("compatible: {:?}", compatible);
        println!("================");
        println!("================");
        println!("incompatible: {:?}", incompatible);
    }

    Ok(())
}
