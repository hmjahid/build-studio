use std::env;
use std::path::Path;

fn print_usage() {
    println!("Build Studio CLI\n");
    println!("Usage:");
    println!("  buildstudio-cli <command> [options]\n");
    println!("Commands:");
    println!("  build <project_dir>      Run build for project");
    println!("  package <project_dir>    Package project artifacts");
    println!("  plugins <plugins_dir>    List available plugins");
    println!("  nodes                    List remote build nodes");
    println!("  help                     Show this help message");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        return;
    }
    match args[1].as_str() {
        "build" => {
            if args.len() < 3 {
                println!("Usage: buildstudio-cli build <project_dir>");
                return;
            }
            let config_path = format!("{}/buildstudio.config.yaml", args[2]);
            let config = match build_studio_lib::config::read_config(config_path.clone()) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("Failed to read config: {}", e);
                    return;
                }
            };
            for build in config.builds {
                println!("Running build: {} (platform: {})", build.name, build.platform);
                match build_studio_lib::build::run_build_no_window(build.command.clone(), args[2].clone(), Some(build.platform.clone())) {
                    Ok(_) => println!("Build '{}' finished successfully.", build.name),
                    Err(e) => eprintln!("Build '{}' failed: {}", build.name, e),
                }
            }
        },
        "package" => {
            if args.len() < 3 {
                println!("Usage: buildstudio-cli package <project_dir>");
                return;
            }
            let config_path = format!("{}/buildstudio.config.yaml", args[2]);
            let config = match build_studio_lib::config::read_config(config_path.clone()) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("Failed to read config: {}", e);
                    return;
                }
            };
            if let Some(pkg) = config.package {
                // Determine the plugins directory (assuming it's in the same parent directory as the project)
                let project_path = Path::new(&args[2]);
                let _plugins_dir = project_path.parent()
                    .map(|parent| parent.join("plugins"))
                    .unwrap_or_else(|| Path::new("./plugins").to_path_buf());
                
                let opts = build_studio_lib::packaging::PackageConfig {
                    name: pkg.name.unwrap_or("app".to_string()),
                    version: pkg.version.unwrap_or("0.1.0".to_string()),
                    package_type: pkg.r#type.unwrap_or("deb".to_string()),
                    dependencies: pkg.dependencies.unwrap_or(vec![]),
                    source_dir: args[2].clone(),
                    output_dir: "./packages".to_string(),
                };
                match build_studio_lib::packaging::create_package(opts) {
                    Ok(msg) => println!("Packaging successful: {}", msg),
                    Err(e) => eprintln!("Packaging failed: {}", e),
                }
            } else {
                println!("No package section in config.");
            }
        },
        "plugins" => {
            let plugins_dir = if args.len() >= 3 {
                args[2].clone()
            } else {
                "./plugins".to_string()
            };
            let plugins = build_studio_lib::plugin::list_plugins(plugins_dir);
            if plugins.is_empty() {
                println!("No plugins found.");
            } else {
                for plugin in plugins {
                    println!("{} ({}): {}", plugin.name, plugin.version, plugin.description);
                }
            }
        },
        "nodes" => {
            println!("Remote node listing only available in GUI for now.");
        },
        "help" | _ => {
            print_usage();
        }
    }
}
