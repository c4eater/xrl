#![allow(unused_variables)]
extern crate futures;
extern crate tokio;
extern crate xrl;

use futures::{Future, Stream};
use xrl::{
    spawn, Client, Frontend, FrontendBuilder, MeasureWidth, XiNotification,
};

// Type that represent our client
struct MyFrontend {
    #[allow(dead_code)]
    client: Client,
}

// Implement how our client handles notifications and requests from the core.
impl Frontend for MyFrontend {
    type NotificationResult = Result<(), ()>;
    fn handle_notification(&mut self, notification: XiNotification) -> Self::NotificationResult {
        use XiNotification::*;

        match notification {
            Update(update) => println!("received `update` from Xi core:\n{:?}", update),
            ScrollTo(scroll) => println!("received `scroll_to` from Xi core:\n{:?}", scroll),
            DefStyle(style) => println!("received `def_style` from Xi core:\n{:?}", style),
            AvailablePlugins(plugins) => {
                println!("received `available_plugins` from Xi core:\n{:?}", plugins)
            }
            UpdateCmds(cmds) => println!("received `update_cmds` from Xi core:\n{:?}", cmds),
            PluginStarted(plugin) => {
                println!("received `plugin_started` from Xi core:\n{:?}", plugin)
            }
            PluginStoped(plugin) => {
                println!("received `plugin_stoped` from Xi core:\n{:?}", plugin)
            }
            ConfigChanged(config) => {
                println!("received `config_changed` from Xi core:\n{:?}", config)
            }
            ThemeChanged(theme) => println!("received `theme_changed` from Xi core:\n{:?}", theme),
            Alert(alert) => println!("received `alert` from Xi core:\n{:?}", alert),
            AvailableThemes(themes) => {
                println!("received `available_themes` from Xi core:\n{:?}", themes)
            }
            FindStatus(status) => println!("received `find_status` from Xi core:\n{:?}", status),
            ReplaceStatus(status) => {
                println!("received `replace_status` from Xi core:\n{:?}", status)
            }
            AvailableLanguages(langs) => {
                println!("received `available_languages` from Xi core:\n{:?}", langs)
            }
            LanguageChanged(lang) => {
                println!("received `language_changed` from Xi core:\n{:?}", lang)
            }
        }
        Ok(())
    }

    type MeasureWidthResult = Result<Vec<Vec<f32>>, ()>;
    fn handle_measure_width(&mut self, request: MeasureWidth) -> Self::MeasureWidthResult {
        Ok(Vec::new())
    }
}

struct MyFrontendBuilder;

impl FrontendBuilder for MyFrontendBuilder {
    type Frontend = MyFrontend;
    fn build(self, client: Client) -> Self::Frontend {
        MyFrontend { client: client }
    }
}

fn main() {
    // spawn Xi core
    let (client, core_stderr) = spawn("xi-core", MyFrontendBuilder {});

    // All clients must send client_started notification first
    tokio::run(client.client_started(None, None).map_err(|_| ()));
    // start logging Xi core's stderr
    let log_core_errors = core_stderr
        .for_each(|msg| {
            println!("xi-core stderr: {}", msg);
            Ok(())
        })
        .map_err(|_| ());
    ::std::thread::spawn(move || {
        tokio::run(log_core_errors);
    });
    // Send a request to open a new view, and print the result
    let open_new_view = client
        .new_view(None)
        .map(|view_name| println!("opened new view: {}", view_name))
        .map_err(|_| ());
    tokio::run(open_new_view);
    // sleep until xi-requests are received
    ::std::thread::sleep(::std::time::Duration::new(5, 0));
}
