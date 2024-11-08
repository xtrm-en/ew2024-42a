use axum::{
    http::{header::CONTENT_TYPE, Method, StatusCode, Uri},
    routing::{get, post},
    Json, Router,
};
use common::{FormContent, News, NewsType};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app().await).await.unwrap();
}

async fn not_found(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("No route for {}", uri))
}

pub async fn app() -> Router {
    let origins = ["http://localhost:8080".parse().unwrap()];
    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::OPTIONS,
            Method::DELETE,
        ])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    Router::new()
        .route("/news", get(get_news))
        .route("/appointment", post(post_appointment))
        .layer(cors)
        .fallback(not_found)
}

async fn get_news() -> Result<Json<Vec<News>>, String> {
    Ok(Json(vec![
        News {
            title: "Exam.".to_string(),
            content: "Exam 02, 03, 04, 05, 06".to_string(),
            r#type: NewsType::Intra42,
        },
        News {
            title: "Joy a réalisé un salto dans la cour de l'école !!!".to_string(),
            content: "Si vous avez raté cette évènement, pas de chance...".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "HTBM".to_string(),
            content: "Internship and Apprenticeship".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "Conference YY Company. It's a startup. It'll be big, like unicorn big. Not much to say about this company, no one knows what they do but they've raised a shit load of money by just spitting the AI keyword in every single pitch sentence. It does web, who would've thought. It needs a frontend + backend + devops + blockchain + AI. Multiple experiences required, 600/monthly salary but don't worry, after that you'll not be paid but you'll get 2% of the company and be our CTO. Gosh sorry this title is long, good luck with that.".to_string(),
            content: "YY need to hire new students. Don't miss out the opportunity.".to_string(),
            r#type: NewsType::CampusEvent,
        },
        News {
            title: "intra v3 is out again, with new bugs introduced".to_string(),
            content: "🐛".to_string(),
            r#type: NewsType::Intra42,
        },
        News {
            title: "rusty v0.0.0.0.0.1 is out, featuring a news page developed by our new team member".to_string(),
            content: "Yesterday, Rusty was a website hastily developed to meet administrative needs and provide basic services.

Today, Rusty is a web platform offering services easing students life and helping them define their professional and educational paths. It is also a means of continuous improvement, giving students a voice heard by the staff.

Tomorrow, Rusty will assist students on all platforms and in all matters related to their journey. It will be a connected hub of information and a comprehensive educational guide, helping them in defining their paths and tracking their progress over time.".to_string(),
            r#type: NewsType::CampusEvent,
        },
        News {
            title: "Improving students experience".to_string(),
            content: "Starting next week, the campus will be open 72/7 to provide students the opportunity to validate common core within a month.".to_string(),
            r#type: NewsType::CampusEvent,
        },
        News {
            title: "Rusty's Getting Rustier as Rust Celebrates its 42nd Birthday!".to_string(),
            content: "It’s hard to believe, but Rust, the language that was once 'the new kid on the block,' is now officially middle-aged. Yes, folks, Rust has reached the grand milestone of its 42nd birthday—a number so significant that it’s likely to mean *something* deep, even if no one can pinpoint exactly what. For those of you who may be unfamiliar with this programming marvel, Rust is the love child of Mozilla and a bunch of developers who were frankly fed up with memory leaks, dangling pointers, and terrifying debugging sessions at 3 a.m. It's the language that looked at other programming languages and said, 'Hold my beer.'
        
        As part of the festivities, Rustaceans (that’s what hardcore Rust fans call themselves, for the uninitiated) have taken to the internet to reminisce over Rust’s early days. One dedicated fan recounted how Rust 0.1 was basically a glorified scientific experiment, resembling more of a Choose Your Own Adventure game than a language. Back then, if you managed to compile anything, you won the programming equivalent of a gold medal, and you probably took the rest of the day off to celebrate.
        
        For those of us on campus who are die-hard Rust fans, we’re marking the occasion by doing what we do best: writing code that’s as safe as it is cool and having lengthy debates on how memory safety is a spiritual experience. It’s a shame there’s no actual cake to cut, though. To remedy this, someone in the IT department has suggested we symbolically ‘leak memory’ in Rust's honor (don’t worry, we’ll fix it by Monday). Meanwhile, campus administration is still wondering what 'Rust' is and why everyone insists on celebrating it like a holiday.
        
        In the coming week, prepare for Rust-themed posters around the campus, new Rusty stickers for laptops, and 'fun facts' posted randomly on Slack. Did you know the name 'Rust' was partly inspired by the oxide that forms on metals? It’s as if Rust's creators knew from the start it would age like fine iron. By next Friday, we’ll also be hosting a 'Rust-Your-Code' session, where students can bring their messiest projects, and we’ll attempt to convert them to safe Rust code. As a final touch, the WiFi password for the month will be changed to 'NoGCMemorySafe42'. 
        
        Here’s to Rust, our beloved language, and to Rusty, our trusty campus app that reminds us of everything great about modern code... even if it gets rustier by the day. Happy Birthday, Rust! May your enums be exhaustive, your borrows non-dangling, and your syntax ever confusing to beginners.".to_string(),
            r#type: NewsType::Intra42,
        },
        News {
            title: "Lost & Found".to_string(),
            content: "A set of keys and a yellow canary were found in the main building.".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "Big news: C1 toilets have stayed clean for 10 minutes straight".to_string(),
            content: "That's dope!".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "Food Truck Fridays Are Back!".to_string(),
            content: "Every Friday, food trucks will be available outside the main building from 11 AM to 2 PM. Enjoy a variety of cuisines!".to_string(),
            r#type: NewsType::CampusEvent,
        },
        News {
            title: "New Study Rooms Available".to_string(),
            content: "We've added more study rooms on the second floor. This feature is now supported thanks to our new elastic and scalable architecture.".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "Campus WiFi Upgrade".to_string(),
            content: "We're upgrading the campus WiFi next week. Expect improved speed and coverage across all areas, including under the babyfoot for Fanny losers.".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "New Course on 'Advanced Procrastination'".to_string(),
            content: "Enroll now to learn the art of productive delay. Seats are expected to fill up... eventually.".to_string(),
            r#type: NewsType::CampusEvent,
        },
        News {
            title: "Mystery of Missing Whiteboard Markers Solved".to_string(),
            content: "Turns out, they've been under the coffee machine the whole time. Case closed!".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "AI Program Now Generating Random Staff Messages".to_string(),
            content: "Because the real news was too normal, an AI is now generating daily nonsense to keep things interesting.".to_string(),
            r#type: NewsType::Intra42,
        },
        News {
            title: "Campus Will Now Charge projects failures in Cryptocurrency".to_string(),
            content: "Pay your dues in Dogecoin or get ready to have your 42 Student badge repossessed.".to_string(),
            r#type: NewsType::CampusEvent,
        },
        News {
            title: "Campus Introduces 'No Coffee' Mondays".to_string(),
            content: "Due to caffeine budget cuts, Mondays will be a coffee-free day. Good luck!".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "IT Dept Announces Plans to Make Error Messages 'More Friendly'".to_string(),
            content: "Starting next week, error screens will come with compliments to boost morale.".to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "New 'Quiet Zone' in C1 Actually Just Soundproof Headphones".to_string(),
            content: "Budget-friendly solution allows students to enter their own world of silence. Bring your own headphones.".to_string(),
            r#type: NewsType::CampusEvent,
        },
        News {
            title: "Student Proposes Installing Beds in Study Rooms".to_string(),
            content: "The initiative was denied, but comfy bean bags are being considered as a compromise.".to_string(),
            r#type: NewsType::CampusEvent,
        },
        News {
            title: "Introducting Dioxus hooks".to_string(),
            content: r#"Most used hooks:
- use_signal(): Declare a new Signal<T>, the closure passed to the function will be run only the first time the component is rendered, and this same closure should return the type T you want your Signal to be. A Signal will be dropped when the component whom declared is dropped.
- use_effect(): A closure that will run once the first time the component is rendered. If you .read() a Signal in it, it will re-run every time the said Signal is written to (ie Subscriber Pattern). It is a good way to way to .write() to a Signal, since it is executed after the rendering and not during, which make it harder to have infinite rendering loops. In 0.5.x you should not read and write to the same Signal in the same use_effect, it will cause infinite loops, but I think it is fixed in 0.6.x.
- use_memo(): Very much like a use_signal() but it will give you something more of a ReadOnlySignal, and with the difference that it will subscribe to every Signal you .read() from in the closure you pass. Very useful when you want to compute something from a Signal, without declaring another Signal and having a use_effect write to it. This hook also comes with some sort of optimization, ie if a Signal you read from changes and the hook reruns but the value stays the same, it will not cause a re-render of the component.
- use_future(): Not used anymore I think, getting replaced by use_resource() ( from https://dioxuslabs.com/learn/0.5/migration/hooks )
- use_resource(): A hook that let you run an async closure and return a Resource. You can .read() from this Resource to get the result of this Resource, Some(Ok(_)) means the async call ended successfully, Some(Err(_ )) means an error, None means it is still loading. The closure you pass to the hook will also subscribe to any Signal you .read() in the closure, causing it to re-run every time is it written to. You can also manually call .restart() on the Resource to manually do it.
More specific hooks:
- use_context_provider(): If passing a Signal as parameter of a component is not feasible or just not nice to do, you can declare a context in the parent and any child will be able to retrieve this context. Note: it is not mandatory make this one a Signal, but I think it should almost always be the case (ie : use_context_provider(|| false) will return a bool versus use_context_provider(|| Signal::new(false) will return a Signal<bool>)
- use_context(): Used to retrieve a context from a parent component, will panic if the context is not found
- try_use_context(): Same as above but will return a Option and not panic!
- use_coroutine(): This hook is very useful when you want to create some sort a background task. In our case we use it to update our global USER. You can see in the fn global_service() that it .await on a message, and when a message comes it will do some action. Our message are in form of Enum. Both use_coroutine and use_coroutine_handle works very much like use_context_provider and use_context, meaning that if you want to send a message you have to retrieve the handle via use_coroutine_handle from a parent component. Then you can call .send() on this handle with the message you want to be sent and treated.
- use_navigator(): Used to progammaticaly navigate the user, ie to make it change page when an action is done or things like that
- use_route(): Used to get information about the current route, like what is the current route the user is rendering"#.to_string(),
            r#type: NewsType::StaffMsg,
        },
        News {
            title: "Announcement: All IT Issues Will Now be Blamed on Mercury Retrograde".to_string(),
            content: "Until further notice, all outages and bugs are attributed to celestial interference.".to_string(),
            r#type: NewsType::Intra42,
        },
    ]))
}

async fn post_appointment(Json(_form_content): Json<FormContent>) -> Result<(), String> {
    Ok(())
}
