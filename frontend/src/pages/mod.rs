use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn HomePage() -> impl IntoView {
    let (category, set_category) = signal("teaching".to_string());
    let (message, set_message) = signal(String::new());
    let (status, set_status) = signal(None::<String>);
    let (submitting, set_submitting) = signal(false);

    let submit_feedback = move |event: leptos::ev::SubmitEvent| {
        event.prevent_default();
        if submitting.get_untracked() {
            return;
        }
        let category = category.get_untracked();
        let message = message.get_untracked();
        set_submitting.set(true);
        set_status.set(Some("Submitting feedback…".into()));

        spawn_local(async move {
            let payload = serde_json::json!({"category": category, "message": message});
            let request = gloo_net::http::Request::post("/api/feedback")
                .header("content-type", "application/json")
                .body(payload.to_string());
            match request {
                Ok(request) => match request.send().await {
                    Ok(response) if response.ok() => {
                        set_status.set(Some("Thank you. Your feedback was submitted.".into()));
                        set_message.set(String::new());
                    }
                    Ok(response) => {
                        set_status.set(Some(format!(
                            "Your feedback could not be submitted (HTTP {}). Please check the form and try again.",
                            response.status()
                        )));
                    }
                    Err(_) => set_status.set(Some("Unable to reach the feedback service.".into())),
                },
                Err(_) => set_status.set(Some("Unable to prepare the feedback request.".into())),
            }
            set_submitting.set(false);
        });
    };

    view! {
        <a class="skip-link" href="#main">"Skip to content"</a>
        <header><a href="#main">"Secure Feedback"</a><span>"Capstone · Development"</span></header>
        <main id="main">
            <p class="eyebrow">"A space for better conversations"</p>
            <h1>"Your feedback can shape what comes next."</h1>
            <p class="intro">"A student feedback platform built with Rust, with privacy and responsible access as design goals."</p>
            <section aria-labelledby="feedback-title">
                <h2 id="feedback-title">"Share feedback"</h2>
                <p class="notice">"Do not include your name, contact details, student number, or other personal information. Development submissions are kept only in memory and are lost when the server restarts."</p>
                <form on:submit=submit_feedback>
                    <label for="category">"Category"</label>
                    <select id="category" name="category" autocomplete="off" on:change=move |event| set_category.set(event_target_value(&event))>
                        <option value="teaching">"Teaching"</option>
                        <option value="course-content">"Course content"</option>
                        <option value="facilities">"Facilities"</option>
                        <option value="other">"Other"</option>
                    </select>
                    <label for="message">"Feedback"</label>
                    <textarea
                        id="message"
                        name="message"
                        aria-describedby="message-help message-count"
                        maxlength="2000"
                        required
                        rows="7"
                        spellcheck="true"
                        on:input=move |event| set_message.set(event_target_value(&event))
                    ></textarea>
                    <div class="field-meta">
                        <small id="message-help">"Keep your feedback specific and respectful."</small>
                        <small id="message-count">{move || format!("{}/2,000 characters", message.get().chars().count())}</small>
                    </div>
                    <button type="submit" disabled=move || submitting.get()>
                        {move || if submitting.get() { "Submitting…" } else { "Submit feedback" }}
                    </button>
                </form>
                <p class="status" aria-live="polite">{move || status.get().unwrap_or_default()}</p>
            </section>
        </main>
        <footer>"Development preview · Please do not enter personal information."</footer>
    }
}
