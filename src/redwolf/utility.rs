use serde_json;
use failure::Error;
use std::collections::BTreeMap;

pub fn extend_json( base: &serde_json::Value, derived: &serde_json::Value ) -> Result< serde_json::Value, Error > {
    // The way this has to be done is completely disgusting
    // Because serde_json is a shitty library

    // Strike 1: Can't copy a serde_json
    let base = serde_json::from_str( &serde_json::to_string( base )? )?;
    let derived = serde_json::from_str( &serde_json::to_string( derived )? )?;

    // Strike 2: Can't iterate a serde_json
    let mut base: BTreeMap< String, serde_json::Value > = serde_json::from_value( base )?;
    let derived: BTreeMap< String, serde_json::Value > = serde_json::from_value( derived )?;

    for ( key, value ) in derived {
        base.insert( key, value );
    }

    Ok( serde_json::to_value( base )? )
}

pub fn copy_json( item: &serde_json::Value ) -> Result< serde_json::Value, Error > {
    Ok( serde_json::from_str( &serde_json::to_string( item )? )? )
}