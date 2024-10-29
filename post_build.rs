fn main() {
    let plg_name = "pza_plugin_std";

    let crate_out_dir = std::env::var("CRATE_OUT_DIR").unwrap();

    panduza_platform_core::env::system_copy_plugin_to_default_location(crate_out_dir, plg_name);
}
