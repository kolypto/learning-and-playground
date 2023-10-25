// Note: in `build.rs` we don't need to explicitly import crates.
// These "[build-dependencies]" are added automatically

fn main() -> anyhow::Result<()> {
    // Prints build args
    embuild::espidf::sysenv::output();

    // Check & import `cfg.toml`
    if !std::path::Path::new("cfg.toml").exists() {
        anyhow::bail!("You need to create a `cfg.toml` file with your Wi-Fi credentials! Use `cfg.toml.example` as a template.");
    }
    let app_config = CONFIG; // const `CONFIG` is auto-generateed
    if app_config.wifi_ssid == "" || app_config.wifi_psk == "" {
        anyhow::bail!("You need to set the Wi-Fi credentials in `cfg.toml`!");
    }

    // // Necessary because of this issue: https://github.com/rust-lang/cargo/issues/9641 :
    // // > "rustc-link-arg does not propagate transitively"
    // // But build actually fails if we enable these
    // embuild::build::CfgArgs::output_propagated("ESP_IDF")?;
    // embuild::build::LinkArgs::output_propagated("ESP_IDF")

    // Return
    Ok(())
}

// App config: the WiFi network to connect to.
// The config is taken from `cfg.toml` and imported into Rust as a value
#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}
