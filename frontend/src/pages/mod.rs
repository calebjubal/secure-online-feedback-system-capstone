use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn HomePage() -> impl IntoView {
    let (category, set_category) = signal("teaching".to_string());
    let (message, set_message) = signal(String::new());
    let (status, set_status) = signal(None::<String>);

    let submit_feedback = move |event: leptos::ev::SubmitEvent| {
        event.prevent_default();
        let category = category.get_untracked();
        let message = message.get_untracked();
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
                    }
                    Ok(response) => {
                        set_status.set(Some(format!(
                            "Your feedback could not be submitted (HTTP {}).",
                            response.status()
                        )));
                    }
                    Err(_) => set_status.set(Some("Unable to reach the feedback service.".into())),
                },
                Err(_) => set_status.set(Some("Unable to prepare the feedback request.".into())),
            }
        });
    };

    view! {
        <header><a href="#main">"Secure Feedback"</a><span>"Capstone · Development"</span></header>
        <main id="main">
            <p class="eyebrow">"A space for better conversations"</p>
            <h1>"Your feedback can shape what comes next."</h1>
            <p class="intro">"A student feedback platform built with Rust, with privacy and responsible access as design goals."</p>
            <section aria-labelledby="feedback-title">
                <h2 id="feedback-title">"Share feedback"</h2>
                <p>"Do not include your name, contact details, student number, or other personal information. Development submissions are kept only in memory and are lost when the server restarts."</p>
                <form on:submit=submit_feedback>
                    <label for="category">"Category"</label>
                    <select id="category" on:change=move |event| set_category.set(event_target_value(&event))>
                        <option value="teaching">"Teaching"</option>
                        <option value="course-content">"Course content"</option>
                        <option value="facilities">"Facilities"</option>
                        <option value="other">"Other"</option>
                    </select>
                    <label for="message">"Feedback"</label>
                    <textarea
                        id="message"
                        maxlength="2000"
                        required
                        rows="7"
                        on:input=move |event| set_message.set(event_target_value(&event))
                    ></textarea>
                    <button type="submit">"Submit feedback"</button>
                </form>
                <p class="status" aria-live="polite">{move || status.get().unwrap_or_default()}</p>
            </section>
        </main>
        <footer>"Development preview · Please do not enter personal information."</footer>
    }
}
