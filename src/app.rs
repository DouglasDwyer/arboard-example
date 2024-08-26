use arboard::*;
use gloo_timers::callback::Interval;
use web_sys::HtmlInputElement;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

static mut CURRENT_TEXT: String = String::new();

fn copy_clipboard_contents() {
    unsafe {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set().text(&*std::ptr::addr_of!(CURRENT_TEXT)).unwrap();
    }
}

fn clear_clipboard_contents() {
    Clipboard::new().unwrap().clear().unwrap();
}

#[function_component(App)]
pub fn app() -> Html {
    unsafe {        
        let node_ref = NodeRef::default();
    
        let oninput = Callback::from(move |input_event: InputEvent| {
            let target: HtmlInputElement = input_event
                .target()
                .unwrap()
                .dyn_into()
                .unwrap();
            *std::ptr::addr_of_mut!(CURRENT_TEXT) = target.value().as_str().to_string();
        });

        let node_ref_clone = node_ref.clone();
        let timeout = Interval::new(100, move || {
            node_ref.cast::<web_sys::HtmlInputElement>().unwrap().set_value(&Clipboard::new().unwrap().get().text().unwrap_or_default());
        });
        
        timeout.forget();
    
        html! {
            <main>
                <text>{"Try pasting anywhere  on the window. Last pasted text: "}</text>
                <br/>
                <input text="text" disabled=true readonly=true ref={node_ref_clone}/>
                <br/>
                <br/>
                <text>{"Type in here and then click the button to copy:"}</text>
                <br/>
                <input type="text" {oninput} />
                <br/>
                <button onclick={Callback::from(|_| copy_clipboard_contents())}>{"Click to copy"}</button>
                <br/>
                <button onclick={Callback::from(|_| clear_clipboard_contents())}>{"Click to clear clipboard"}</button>
            </main>
        }
    }
}
