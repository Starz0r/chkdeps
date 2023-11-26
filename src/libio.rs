use {anyhow::Result, serde::Deserialize};

#[derive(Deserialize)]
pub struct LibIO {
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct Library {
    pub latest_release_number: String,
    pub latest_stable_release_number: String,
}

pub fn get_lib(platform: &str, name: &str, api_key: &str) -> Result<Library> {
    let req = ureq::get(
        &("https://libraries.io/api/".to_owned() + platform + "/" + name + "?api_key=" + api_key),
    )
    .call()?;
    let lib = serde_json::from_reader(req.into_reader())?;
    Ok(lib)
}
