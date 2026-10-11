// byc_driver::config — Configuration parsing (by.toml)

pub struct Config {
    pub project_name: String,
    pub version: String,
    pub optimization: String,
    pub dependencies: Vec<String>,
}
