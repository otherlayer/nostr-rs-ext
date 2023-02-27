use std::fs;
use libloading::{Library, Symbol};
use std::path::{Path, PathBuf};
use nostr_rs_plugin::Plugin;
use tracing::debug;

use nostr_rs_proto::nauthz_grpc::{EventReply, EventRequest};

fn get_plugin(path: PathBuf) -> ExtPlugin {
    unsafe {
        let lib = 
            Library::new(&path)
            .ok()
            .expect(format!("Can't load plugin at path: {}", path.display()).as_str());
        
        let func: Symbol<unsafe extern fn() -> Box<dyn Plugin + Send + Sync>> = 
            lib.get(b"get_plugin")
            .ok()
            .expect("Implement 'get_plugin' in plugin library (check example)");

        let plugin = func();

        let ext_plugin = ExtPlugin {
            lib,
            plugin
        };

        ext_plugin
    }
}

pub struct ExtPlugin {
    #[allow(dead_code)]
    // holds library reference
    pub lib: Library,
    pub plugin: Box<dyn Plugin + Send + Sync>,
}

impl Plugin for ExtPlugin {
    fn start(&self) {}

    fn name(&self) -> String {
        return self.plugin.name();
    }

    fn admit_event(&self, request: &EventRequest) -> EventReply {
        return self.plugin.admit_event(request);
    }

    fn stop(&self) {}
}

pub async fn load_plugins(plugins_folder: String) -> Vec<ExtPlugin> {
    let mut plugins: Vec<ExtPlugin> = Vec::new();

    if let Ok(paths) = fs::read_dir(&plugins_folder) {
        for dir in paths {
            let dir_path = dir.ok().unwrap().path().display().to_string();

            let path = Path::new(&dir_path).join("libplugin.so");

            let plugin = get_plugin(path);

            let n = plugin.name();

            debug!("Plugin: {} loaded", n);

            plugins.push(plugin);
        }
    }

    plugins
}
