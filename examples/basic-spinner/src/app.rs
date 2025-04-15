use leptos::either::Either;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    SsrMode, StaticSegment, WildcardSegment,
};
use serde::{Deserialize, Serialize};
use spux::pulsers::Circle;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/basic-spinner.css"/>

        // sets the document title
        <Title text="Welcome to Leptos with Spux"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage ssr=SsrMode::OutOfOrder/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // setting up the posts resource to call onto the server function get_posts()
    let posts = Resource::new(|| (), move |_| async move { get_posts().await });

    // retrieving the posts and rendering them in the view below
    let posts_view = move || {
        posts.get().map(|all_posts_result| match all_posts_result {
            Ok(all_posts) => Either::Left(
                all_posts
                    .iter()
                    .map(|each_post: &PostData| {
                        view! {
                            <p>
                                <div>"Title: "{each_post.title.clone()}</div>
                                <div>"Content: "{each_post.content.clone()}</div>
                            </p>
                        }
                    })
                    .collect_view(),
            ),
            Err(_) => Either::Right(view! {
                <div>"error with post retrieval"</div>
            }),
        })
    };

    view! {
        <Suspense fallback=move || view! {
            <div
                style:width="full"
                style:margin-x="auto"
                style:align-items="center"
                style:justify-content="center"
                style:display="flex"
            >
                <Circle color="#000000" size=12 />
            </div>
        }>
            <div>
                <p>"Posts"</p>
                <hr />
                {posts_view}
            </div>
        </Suspense>
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PostData {
    id: i32,
    title: String,
    content: String,
}

#[server]
pub async fn get_posts() -> Result<Vec<PostData>, ServerFnError> {
    // we intentionally set a 3 sec delay to showcase the pulsating
    // circle in action
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    let posts: Vec<PostData> = vec![
        PostData {
            id: 0,
            title: "Post #1".to_string(),
            content: "This is the 1st post".to_string(),
        },
        PostData {
            id: 1,
            title: "Post #2".to_string(),
            content: "This is the 2nd post".to_string(),
        },
        PostData {
            id: 2,
            title: "Post #3".to_string(),
            content: "This is the 3rd post".to_string(),
        },
    ];

    Ok(posts)
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
