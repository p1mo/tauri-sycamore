use serde::{Deserialize, Serialize};
//use serde_wasm_bindgen::to_value;
use sycamore::futures::spawn_local_scoped;
use sycamore::prelude::*;
use sycamore::rt::Event;
//use wasm_bindgen::prelude::*;
use futures_util::StreamExt;

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}


#[derive(Serialize, Deserialize)]
struct GreetEmit<'a> {
    msg: &'a str,
}

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {

    // TEST OS PLUGIN

    let os_arch = create_signal(cx, String::new());
    let os_exe_extension = create_signal(cx, String::new());
    let os_family = create_signal(cx, String::new());
    let os_hostname = create_signal(cx, String::new());
    let os_locale = create_signal(cx, String::new());
    let os_platform = create_signal(cx, String::new());
    let os_kind = create_signal(cx, String::new());
    let os_version = create_signal(cx, String::new());

    spawn_local_scoped(cx, async move {

        let arch = tauri_wasm::plugin::os::arch().await.unwrap();

        match arch {
            tauri_wasm::plugin::os::Arch::X86       => os_arch.set("X86".to_string()),
            tauri_wasm::plugin::os::Arch::X86_64    => os_arch.set("X86_64".to_string()),
            tauri_wasm::plugin::os::Arch::Arm       => os_arch.set("Arm".to_string()),
            tauri_wasm::plugin::os::Arch::Aarch64   => os_arch.set("Aarch64".to_string()),
            tauri_wasm::plugin::os::Arch::Mips      => os_arch.set("Mips".to_string()),
            tauri_wasm::plugin::os::Arch::Mips64    => os_arch.set("Mips64".to_string()),
            tauri_wasm::plugin::os::Arch::Powerpc   => os_arch.set("Powerpc".to_string()),
            tauri_wasm::plugin::os::Arch::Powerpc64 => os_arch.set("Powerpc64".to_string()),
            tauri_wasm::plugin::os::Arch::Riscv64   => os_arch.set("Riscv64".to_string()),
            tauri_wasm::plugin::os::Arch::S390x     => os_arch.set("S390x".to_string()),
            tauri_wasm::plugin::os::Arch::Sparc64   => os_arch.set("Sparc64".to_string()),
        };

        let exe_extension = tauri_wasm::plugin::os::exe_extension().await.unwrap();
        os_exe_extension.set(exe_extension);

        let family = tauri_wasm::plugin::os::family().await.unwrap();
        os_family.set(family);

        let hostname = tauri_wasm::plugin::os::hostname().await.unwrap();
        os_hostname.set(hostname);

        let locale = tauri_wasm::plugin::os::locale().await.unwrap();
        os_locale.set(locale);   

        let platform = tauri_wasm::plugin::os::platform().await.unwrap();

        match platform {
            tauri_wasm::plugin::os::Platform::Linux     => os_platform.set("Linux".to_string()),
            tauri_wasm::plugin::os::Platform::Macos     => os_platform.set("Macos".to_string()),
            tauri_wasm::plugin::os::Platform::Ios       => os_platform.set("Ios".to_string()),
            tauri_wasm::plugin::os::Platform::Freebsd   => os_platform.set("Freebsd".to_string()),
            tauri_wasm::plugin::os::Platform::Dragonfly => os_platform.set("Dragonfly".to_string()),
            tauri_wasm::plugin::os::Platform::Netbsd    => os_platform.set("Netbsd".to_string()),
            tauri_wasm::plugin::os::Platform::Openbsd   => os_platform.set("Openbsd".to_string()),
            tauri_wasm::plugin::os::Platform::Solaris   => os_platform.set("Solaris".to_string()),
            tauri_wasm::plugin::os::Platform::Android   => os_platform.set("Android".to_string()),
            tauri_wasm::plugin::os::Platform::Windows   => os_platform.set("Windows".to_string()),
        };

        let kind = tauri_wasm::plugin::os::kind().await.unwrap();

        match kind {
            tauri_wasm::plugin::os::OsKind::Linux       => os_kind.set("Linux".to_string()),
            tauri_wasm::plugin::os::OsKind::Windows     => os_kind.set("Windows".to_string()),
            tauri_wasm::plugin::os::OsKind::Macos       => os_kind.set("Macos".to_string()),
            tauri_wasm::plugin::os::OsKind::Ios         => os_kind.set("Ios".to_string()),
            tauri_wasm::plugin::os::OsKind::Android     => os_kind.set("Android".to_string()),
        };

        let version = tauri_wasm::plugin::os::version().await.unwrap();
        os_version.set(version);    

    });


    // TEST CORE

    let input_invoke = create_signal(cx, String::new());
    let msg_invoke = create_signal(cx, String::new());

    let test_invoke = move |e: Event| {
        e.prevent_default();
        spawn_local_scoped(cx, async move {
            let message : String = tauri_wasm::api::core::invoke("greet", &GreetArgs { name: &input_invoke.get() }).await.unwrap();
            tauri_wasm::js::console::log(&message);
            msg_invoke.set(message);
        })
    };
    

    // TEST EVENT

    let input_emit = create_signal(cx, String::new());
    let msg_emit = create_signal(cx, String::new());

    spawn_local_scoped(cx, async move {

        let mut events = tauri_wasm::api::event::listen::<String>("testrevent").await.expect("event listen error");

        while let Some(event) = events.next().await {

            let payload : String = event.payload;

            let message = format!("payload: {}", payload);

            tauri_wasm::js::console::log(&message);

            msg_emit.set(message);

        }

    });

    let test_emit = move |e: Event| {
        e.prevent_default();
        spawn_local_scoped(cx, async move {
            let message = &GreetEmit { msg: &input_emit.get() };
            tauri_wasm::js::console::log(message.msg);
            tauri_wasm::api::event::emit("testevent", message).await.expect("event emit error");
        });
    };


    // TEST API APP

    let app_get_name = create_signal(cx, String::new());
    let app_get_version = create_signal(cx, String::new());
    let app_get_tauri_version = create_signal(cx, String::new());

    spawn_local_scoped(cx, async move {

        let get_name = tauri_wasm::api::app::get_name().await.unwrap();
        app_get_name.set(get_name);

        let get_version = tauri_wasm::api::app::get_version().await.unwrap();
        app_get_version.set(format!("{}.{}.{}", get_version.major, get_version.minor, get_version.patch));

        let get_tauri_version = tauri_wasm::api::app::get_tauri_version().await.unwrap();
        app_get_tauri_version.set(format!("{}.{}.{}", get_tauri_version.major, get_tauri_version.minor, get_tauri_version.patch));

    });

    let app_hide = move |e: Event| {
        e.prevent_default();
        spawn_local_scoped(cx, async move {
            tauri_wasm::js::console::log("test: app_hide");
            tauri_wasm::api::app::hide().await.expect("app_hide error");
        });
    };

    let app_show = move |e: Event| {
        e.prevent_default();
        spawn_local_scoped(cx, async move {
            tauri_wasm::js::console::log("test: app_show");
            tauri_wasm::api::app::show().await.expect("app_show error");
        });
    };


    // TEST API PATH

    let path_app_config_dir = create_signal(cx, String::new());
    let path_app_data_dir = create_signal(cx, String::new());
    let path_app_local_data_dir = create_signal(cx, String::new());
    let path_app_cache_dir = create_signal(cx, String::new());
    let path_audio_dir = create_signal(cx, String::new());

    spawn_local_scoped(cx, async move {

        let app_config_dir = tauri_wasm::api::path::app_config_dir().await.unwrap();
        path_app_config_dir.set(app_config_dir.display().to_string());

        let app_data_dir = tauri_wasm::api::path::app_data_dir().await.unwrap();
        path_app_data_dir.set(app_data_dir.display().to_string());

        let app_local_data_dir = tauri_wasm::api::path::app_local_data_dir().await.unwrap();
        path_app_local_data_dir.set(app_local_data_dir.display().to_string());

        let app_cache_dir = tauri_wasm::api::path::app_cache_dir().await.unwrap();
        path_app_cache_dir.set(app_cache_dir.display().to_string());

        let audio_dir = tauri_wasm::api::path::audio_dir().await.unwrap();
        path_audio_dir.set(audio_dir.display().to_string());

    });


    // TEST PLUGIN FS

    let exists_data_dir = create_signal(cx, String::new());

    spawn_local_scoped(cx, async move {

        let data_dir = tauri_wasm::api::path::app_data_dir().await.unwrap();

        let path = std::path::Path::new(&data_dir);

        tauri_wasm::js::console::log(&format!("data_dir: {}", path.display()).to_string());

        let edd = tauri_wasm::plugin::fs::exists(&path, tauri_wasm::plugin::fs::BaseDirectory::AppData).await;

        match edd {
            Ok(res) => {

                tauri_wasm::js::console::log(&format!("[FS] {}", res));

                exists_data_dir.set(res.to_string());

            },
            Err(error) => {

                tauri_wasm::js::console::error(&format!("[FS] Error: {}", error));

                exists_data_dir.set("Error: ".to_string());

            },            
        };

    });






    view! { cx,
        div(class="header") {
            a(href="https://beta.tauri.app",target="_blank",class="logo-link") {
                img(src="public/tauri.svg",class="logo",alt="Tauri logo")
            }
            a(href="https://sycamore-rs.netlify.app",class="logo-link",target="_blank") {
                img(src="public/sycamore.svg",class="logo",alt="Sycamore logo")
            }
            a(href="https://webassembly.org",class="logo-link",target="_blank") {
                img(src="public/wasm.svg",class="logo",alt="WASM logo")
            }
            a(href="https://github.com/p1mo/tauri-wasm",class="logo-link",target="_blank") {
                span(class="logo-text"){
                    span(class="font-tauri text-tauri"){ "TAURI" }
                    span(class="font-wasm text-wasm"){ "WASM" }  
                }
            }
        }

        main() {

            div(class="panel-3") {
                div() {
                    span(class="panel-title") { "OS Info" }
                    p(class="line") { b{ "Arch: " }       i{ (os_arch.get()) } }
                    p(class="line") { b{ "Extension: " }  i{ (os_exe_extension.get()) } }
                    p(class="line") { b{ "Family: " }     i{ (os_family.get()) } }
                    p(class="line") { b{ "Hostname: " }   i{ (os_hostname.get()) } }
                    p(class="line") { b{ "Locale: " }     i{ (os_locale.get()) } }
                    p(class="line") { b{ "Platform: " }   i{ (os_platform.get()) } }
                    p(class="line") { b{ "Kind: " }       i{ (os_kind.get()) } }
                    p(class="line") { b{ "Version: " }    i{ (os_version.get()) } }
                }
                div() {
                    span(class="panel-title") { "Core Invoke" }
                    form(class="test_invoke",on:submit=test_invoke) {
                        input(id="greet-invoke",bind:value=input_invoke,placeholder="Enter...",autocomplete="off")
                        button(type="submit") { "Greet Invoke" }
                    }
                    p(class="messages") { b { (msg_invoke.get()) } }
                    
                    span(class="panel-title") { "Event Emit/Listen" }
                    form(class="test_emit",on:submit=test_emit) {
                        input(id="greet-emit",bind:value=input_emit,placeholder="Enter...",autocomplete="off")
                        button(type="submit") { "Greet Emit" }
                    }
                    p(class="messages") { b { (msg_emit.get()) } }
                }

                div() {

                    span(class="panel-title") { "API APP" }
                    p(class="line") { b{ "Name: " } i{ (app_get_name.get()) } }
                    p(class="line") { b{ "Version: " } i{ (app_get_version.get()) } }
                    p(class="line") { b{ "Tauri Version: " } i{ (app_get_tauri_version.get()) } }
                    p(class="line") { button(type="button",on:click=app_hide) { "App Hide" } button(type="button",on:click=app_show) { "App Show" } }

                    span(class="panel-title") { "Plugin FS" }
                    p(class="line") { b{ "exists: " } i{ (exists_data_dir.get()) } }

                }

            }

            div(class="panel-1") {

                div() {
                    span(class="panel-title") { "Path Info" }
                    p(class="line") { b{ "config_dir: " } i{ (path_app_config_dir.get()) } }
                    p(class="line") { b{ "data_dir: " } i{ (path_app_data_dir.get()) } }
                    p(class="line") { b{ "local_data_dir: " } i{ (path_app_local_data_dir.get()) } }
                    p(class="line") { b{ "cache_dir: " } i{ (path_app_cache_dir.get()) } }
                    p(class="line") { b{ "audio_dir: " } i{ (path_audio_dir.get()) } }
                }

            }

        }

    }
}
