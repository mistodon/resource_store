use std::env::current_exe;
use std::io::Result;
use std::path::PathBuf;

mod store;

pub use self::store::*;


pub fn path_to_asset(asset_name: &str) -> Result<PathBuf>
{
    if cfg!(debug_assertions)
    {
        debug_path_to_asset(asset_name)
    }
    else
    {
        release_path_to_asset(asset_name)
    }
}


fn debug_path_to_asset(asset_name: &str) -> Result<PathBuf>
{
    let mut path = current_exe()?;
    path.pop();
    path.pop();
    path.pop();
    path.push("assets");
    path.push(asset_name);
    Ok(path)
}


fn release_path_to_asset(asset_name: &str) -> Result<PathBuf>
{
    let mut path = current_exe()?;
    path.pop();
    path.push("assets");
    path.push(asset_name);
    Ok(path)
}