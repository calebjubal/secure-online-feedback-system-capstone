use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <header><a href="#main">"Secure Feedback"</a><span>"Capstone · Development"</span></header>
        <main id="main">
            <p class="eyebrow">"A space for better conversations"</p>
            <h1>"Your feedback can shape what comes next."</h1>
            <p class="intro">"A student feedback platform built with Rust, with privacy and responsible access as design goals."</p>
            <section aria-labelledby="status-title">
                <h2 id="status-title">"We’re building the foundations"</h2>
                <p>"Feedback submission and administrator access are not available yet. This starter does not collect or store your information."</p>
                <ul>
                    <li>"Student feedback workflows — planned"</li>
                    <li>"Authorized administrative review — planned"</li>
                    <li>"Retention and anonymity policies — to be defined"</li>
                </ul>
            </section>
        </main>
        <footer>"Development preview · Please do not enter personal information."</footer>
    }
}
