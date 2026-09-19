//! Routes and shared page layout.
use crate::components::mark::Mark;
use crate::pages::{
    goals::Goals, method::Method, not_found::NotFound, overview::Overview, record::Record,
    results::Results,
};
use crate::util::scroll_to_top;
use leptos::prelude::*;
use leptos::{ev, html};
use leptos_router::components::{Redirect, Route, Router, Routes};
use leptos_router::hooks::use_location;
use leptos_router::{path, NavigateOptions};
use loopseed_record::{ORGANISATION, ORGANISATION_URL, REPOSITORY_URL};
use wasm_bindgen::JsCast;

pub const NAV: &[(&str, &str)] = &[
    ("/", "Home"),
    ("/results", "Results"),
    ("/method", "Method"),
    ("/record", "Reports"),
    ("/goals", "Research goals"),
];

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <a class="skip-link" href="#main">"Skip to content"</a>
            <Header/>
            <main id="main" class="main" tabindex="-1">
                <Routes fallback=|| view! { <NotFound/> }>
                    <Route path=path!("/") view=Overview/>
                    <Route path=path!("/results") view=Results/>
                    <Route path=path!("/method") view=Method/>
                    <Route path=path!("/record") view=Record/>
                    <Route path=path!("/goals") view=Goals/>
                    <Route path=path!("/promise") view=LegacyGoals/>
                </Routes>
            </main>
            <Footer/>
            <ScrollKeeper/>
        </Router>
    }
}

/// Preserve links to the previous research-goals address, including section anchors.
#[component]
fn LegacyGoals() -> impl IntoView {
    let location = use_location();
    let path = format!("/goals{}", location.hash.get_untracked());
    view! { <Redirect path=path options=NavigateOptions { replace: true, ..Default::default() }/> }
}

#[component]
fn Header() -> impl IntoView {
    let pathname = use_location().pathname;
    let (menu_open, set_menu_open) = signal(false);
    let header = NodeRef::<html::Header>::new();
    let toggle = NodeRef::<html::Button>::new();

    Effect::new(move |_| {
        pathname.track();
        set_menu_open.set(false);
    });

    let outside_click = window_event_listener(ev::pointerdown, move |event| {
        if menu_open.get_untracked() {
            let target = event
                .target()
                .and_then(|target| target.dyn_into::<web_sys::Node>().ok());
            if let (Some(header), Some(target)) = (header.get(), target) {
                if !header.contains(Some(&target)) {
                    set_menu_open.set(false);
                }
            }
        }
    });
    let resize = window_event_listener(ev::resize, move |_| {
        if window()
            .inner_width()
            .ok()
            .and_then(|width| width.as_f64())
            .is_some_and(|width| width > 820.0)
        {
            set_menu_open.set(false);
        }
    });
    on_cleanup(move || {
        outside_click.remove();
        resize.remove();
    });

    view! {
        <header
            class="site-header"
            node_ref=header
            on:keydown=move |event| {
                if event.key() == "Escape" && menu_open.get_untracked() {
                    event.prevent_default();
                    set_menu_open.set(false);
                    if let Some(toggle) = toggle.get() { let _ = toggle.focus(); }
                }
            }
            on:focusout=move |event| {
                let target = event.related_target().and_then(|target| target.dyn_into::<web_sys::Node>().ok());
                if let (Some(header), Some(target)) = (header.get(), target) {
                    if !header.contains(Some(&target)) {
                        set_menu_open.set(false);
                    }
                }
            }
        >
            <div class="wrap header-row">
                <a class="brand" href="/" aria-label="Loopseed home" on:click=move |_| set_menu_open.set(false)>
                    <Mark/>
                    <span class="brand-name">"Loopseed"</span>
                </a>
                <button
                    class="nav-toggle"
                    type="button"
                    node_ref=toggle
                    aria-controls="primary-navigation"
                    aria-expanded=move || menu_open.get().to_string()
                    aria-label=move || if menu_open.get() { "Close menu" } else { "Open menu" }
                    on:click=move |_| set_menu_open.update(|open| *open = !*open)
                >
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8" stroke-linecap="round" aria-hidden="true">
                        <path class="menu-icon" d="M4 6h16M4 12h16M4 18h16"/>
                        <path class="close-icon" d="m6 6 12 12M6 18 18 6"/>
                    </svg>
                </button>
                <nav id="primary-navigation" class="nav" class:nav-open=move || menu_open.get() aria-label="Primary">
                    {NAV.iter().map(|(href, label)| {
                        let href = *href;
                        let label = *label;
                        let current = move || {
                            let path = pathname.get();
                            if href == "/" { path == "/" } else { path.starts_with(href) }
                        };
                        view! {
                            <a href=href aria-current=move || current().then_some("page") on:click=move |_| set_menu_open.set(false)>{label}</a>
                        }
                    }).collect_view()}
                </nav>
            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="wrap footer-grid">
                <div class="footer-brand">
                    <Mark/>
                    <div>
                        <div class="brand-name">"Loopseed"</div>
                        <p class="footer-tag">"Research on systems that learn from interaction, by " <a href=ORGANISATION_URL>{ORGANISATION}</a> "."</p>
                    </div>
                </div>
                <div class="footer-cols">
                    <div>
                        <div class="footer-h">"Read"</div>
                        {NAV.iter().map(|(href, label)| view! { <a href=*href>{*label}</a> }).collect_view()}
                    </div>
                    <div>
                        <div class="footer-h">"Evidence"</div>
                        {match REPOSITORY_URL {
                            Some(url) => view! { <a href=url>"Repository"</a> }.into_any(),
                            None => view! { <span class="footer-muted">"Repository private during the study; evidence on request"</span> }.into_any(),
                        }}
                        <a href="/record">"Reports and source records"</a>
                    </div>
                    <div>
                        <div class="footer-h">"This site"</div>
                        <div class="footer-meta mono">
                            <div>"Research record · 11 September 2026"</div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="wrap footer-line mono">
                <span>{format!("© {ORGANISATION} · Loopseed")}</span>
            </div>
        </footer>
    }
}

/// Scroll to the fragment, or the top when none is present.
#[component]
fn ScrollKeeper() -> impl IntoView {
    let location = use_location();
    let pathname = location.pathname;
    let hash = location.hash;
    Effect::new(move |_| {
        pathname.track();
        let fragment = hash.get();
        let id = fragment.trim_start_matches('#').to_string();
        if id.is_empty() {
            scroll_to_top();
            return;
        }
        fn attempt(id: String, tries_left: u32) {
            match document().get_element_by_id(&id) {
                Some(element) => element.scroll_into_view(),
                None if tries_left > 0 => set_timeout(
                    move || attempt(id, tries_left - 1),
                    std::time::Duration::from_millis(80),
                ),
                None => {}
            }
        }
        attempt(id, 12);
    });
    view! { <></> }
}
