use stdweb::{
    self,
    web::{
        document,
        event::ClickEvent,
        Element,
        HtmlElement,
    },
    js,
    _js_impl,
    traits::*,
    unstable::TryInto,
};
use forge::{
    Engine,
    Io,
    ExecResult,
};

struct DemoIo {
    output: String,
}

impl DemoIo {
    pub fn new() -> Self {
        Self {
            output: String::new(),
        }
    }
}

impl Io for DemoIo {
    fn input(&mut self, msg: String) -> ExecResult<String> {
        let input = js! {
            return window.prompt(@{msg}, "");
        };
        Ok(if let stdweb::Value::String(s) = input {
            s
        } else {
            "null".to_string()
        })
    }

    fn print(&mut self, msg: String) -> ExecResult<()> {
        js! {
            document.getElementById("output").value += @{msg} + "\n";
        }
        Ok(())
    }
}

fn main() {
    stdweb::initialize();

    document()
        .document_element()
        .unwrap()
        .append_html(include_str!("index.html"))
        .unwrap();
    document()
        .head()
        .unwrap()
        .append_html(&("<style>".to_string() + include_str!("style.css") + "</style>"))
        .unwrap();
    document()
        .head()
        .unwrap()
        .append_html(&("<script>".to_string() + include_str!("script.js") + "</script>"))
        .unwrap();

    let default_code = include_str!("default.fg");

    js! {
        document.getElementsByClassName("input")[0].value = @{default_code};
    };

    let on_execute = move || {
        let code = if let stdweb::Value::String(s) = js! {
            return document.getElementById("input").value;
        } {
            s
        } else {
            String::new()
        };

        let _ = Engine::build()
            .with_io(DemoIo::new())
            .finish()
            .exec(&code)
            .map_err(|err| {
                let err_msg = format!("{}", err);
                js! {
                    document.getElementById("output").value += @{err_msg};
                }
            });
    };

    js! {
        document.getElementById("execute").onclick = function() { @{on_execute}(); };
    }
}
