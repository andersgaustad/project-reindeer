use clap::Parser;


#[derive(Debug, Parser)]
#[command(version, about, propagate_version = true, long_about = None)]
pub struct ClapParser {
    /// Version string (x.y.z)
    #[arg(short, long, value_parser = validate_tag)]
    pub tag : String,
}


fn validate_tag(input : &str) -> Result<String, String> {
    let error_fn = || {
        Err("Invalid format! Must be on the form 'x.y.z'.".to_string())
    };

    let split = input.split(".");

    let triplet_opt : Result<[&str; 3], _> = split.collect::<Vec<_>>().try_into();
    let Ok(triplet) = triplet_opt else {
        return error_fn();
    };

    let any_empty_strings = triplet
        .iter()
        .any(|item| item.is_empty());

    if any_empty_strings {
        return error_fn();
    }

    // Else:
    Ok(input.to_string())
}
