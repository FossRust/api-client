use std::time::Duration;

use floem::{
    IntoView, action,
    prelude::{SignalGet, SignalUpdate},
    reactive::{RwSignal, create_rw_signal},
    views::{Decorators, button, h_stack, label, scroll, text_input, v_stack},
};

fn app_view() -> impl IntoView {
    // reactive state
    let method: RwSignal<String> = create_rw_signal("GET".to_string());
    let url: RwSignal<String> = create_rw_signal("https://httpbin.org/get".to_string());
    let body: RwSignal<String> = create_rw_signal(String::new());
    let sending = create_rw_signal(false);
    let resp = create_rw_signal(String::new());

    // send action (spawns a worker thread; posts result back to UI thread)
    let send_click = {
        let method = method;
        let url = url;
        let body = body;
        let sending = sending;
        let resp = resp;
        move || {
            if sending.get() {
                return;
            }
            sending.set(true);

            let m = method.get();
            let u = url.get();
            let b = body.get();

            std::thread::spawn(move || {
                // blocking reqwest keeps it simple and avoids a runtime here
                let client = reqwest::blocking::Client::new();
                let res = match m.as_str() {
                    "POST" => client.post(&u).body(b).send(),
                    "PUT" => client.put(&u).body(b).send(),
                    "DELETE" => client.delete(&u).send(),
                    _ => client.get(&u).send(),
                };

                let out = match res {
                    Ok(r) => {
                        let status = r.status();
                        let headers = format!("{:#?}", r.headers());
                        let text = r.text().unwrap_or_default();
                        format!("Status: {status}\n\n{headers}\n\n{text}")
                    }
                    Err(e) => format!("Error: {e}"),
                };

                // hop back to UI thread before touching signals
                action::exec_after(Duration::from_millis(0), move |_| {
                    resp.set(out);
                    sending.set(false);
                });
            });
        }
    };

    v_stack((
        // top row: method, url, send
        h_stack((
            label(|| "Method:".to_string()),
            text_input(method).style(|s| s.width(100.0)),
            label(|| "URL:".to_string()),
            text_input(url).style(|s| s.width(420.0)),
            button(label(move || {
                if sending.get() {
                    "Sending...".to_string()
                } else {
                    "Send".to_string()
                }
            }))
            .action(send_click)
            .disabled(move || sending.get()),
        ))
        .style(|s| s.gap(8.0)),
        // simple body (one line for now; swap to text_editor(Rope) later if you want multiline)
        label(|| "Body:".to_string()),
        text_input(body).style(|s| s.width(620.0)),
        // response
        label(|| "Response:".to_string()),
        scroll(label(move || resp.get())).style(|s| s.height(260.0)),
    ))
    .style(|s| s.gap(10.0).padding(10.0))
}

fn main() {
    floem::launch(app_view);
}
