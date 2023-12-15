#[cfg(feature = "ssr")]
#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    use std::path::Path;
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use {{crate_name}}::app::*;
    use {{crate_name}}::fileserv::file_and_error_handler;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(Some("Cargo.toml")).await.unwrap();
    let leptos_options = conf.leptos_options;

    let site_pkg_dir = format!("{}/{}",  &leptos_options.site_root, &leptos_options.site_pkg_dir);
    let wasm_path = format!("{}/{}.wasm", &site_pkg_dir, &leptos_options.output_name);
    let should_rename_wasm = Path::new(&wasm_path).exists();
    if should_rename_wasm {
        std::fs::rename(wasm_path, format!("{}/{}_bg.wasm", &site_pkg_dir, &leptos_options.output_name))?;
    }

    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    Ok(app.into())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
