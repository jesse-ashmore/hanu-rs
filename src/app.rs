use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

pub type StoryId = u32;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/hanu-rs.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[server]
pub async fn top_stories() -> Result<Vec<crate::api::StoryId>, ServerFnError> {
    let client = crate::api::Client::new("hacker-news.firebaseio.com").unwrap();
    let stories = client.top_stories().get_all().await.map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(stories)
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let stories_resource = OnceResource::new(top_stories());

    view! {
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            { move || {
                match stories_resource.get() {
                    Some(Ok(stories)) => view! { <StoryList stories /> }.into_any(),
                    Some(Err(err)) => view! { <p>{err.to_string()}</p> }.into_any(),
                    None => view! { <p>{"Loading...".to_string()}</p> }.into_any(),
                }
            }}
        </Suspense>
    }
}

/// Displays a list of stories.
#[component]
fn StoryList(stories: Vec<StoryId>) -> impl IntoView {
    view! {
        <ul>
            <For
                each=move || stories.clone()
                key=|story| story.clone()
                children=move |story| {
                    view! { <li>{story.to_string()}</li> }
                }
            />
        </ul>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
