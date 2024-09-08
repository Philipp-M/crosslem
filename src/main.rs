#![feature(type_alias_impl_trait)]

#[cfg(all(feature = "native", feature = "web"))]
compile_error!("features `native` and `web` are mutually exclusive");

#[cfg(feature = "native")]
use xilem::{view::prose, EventLoop, WidgetView, Xilem};
#[cfg(feature = "web")]
use xilem_web::{document_body, elements::html::p, App, DomView};

struct MyApp;

#[cfg(feature = "web")]
type MyView = impl DomView<MyApp>;
#[cfg(feature = "native")]
type MyView = impl WidgetView<MyApp>;

impl MyApp {
    fn view(&mut self) -> MyView {
        #[cfg(feature = "web")]
        let v = p("some text");
        #[cfg(feature = "native")]
        let v = prose("some text");
        v
    }

    fn run(self) {
        #[cfg(feature = "native")]
        Xilem::new(self, MyApp::view)
            .run_windowed(EventLoop::with_user_event(), "Crosslem".into())
            .unwrap();

        #[cfg(feature = "web")]
        {
            console_error_panic_hook::set_once();
            App::new(document_body(), self, MyApp::view).run();
        }
    }
}

pub fn main() {
    MyApp.run()
}
