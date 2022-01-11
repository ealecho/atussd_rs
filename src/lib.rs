use serde::Deserialize;
use worker::*;

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    
    #[derive(Deserialize)]
    struct AfTalkRequest {
        sessionId: String,
        phoneNumber: String,
        networkCode: String,
        serviceCode: String,
        text: String,
    }


    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .post_async("/test", |mut req, _ctx| async move {

            let data: AfTalkRequest;
            match req.json().await {
                Ok(res) => data = res,
                Err(_) => return Response::error("Bad request", 400),
            }
            
            console_log!("{}",data.text);
            
       
            match data.text.as_str() {
                "" => Response::ok(format!("CON What would you like to check
                1. My account
                2. My phone number")),

                "1" => Response::ok( format!("
                CON Choose account information you want to view
                1. My Account")),

                "2" => Response::ok(format!("END Your phone number is {}", data.phoneNumber)),

                "1*1" => {
                let account_number = "AS123";
                Response::ok(format!("
                END Your account number is {}
                ",account_number))
                },

                _ => Response::error("Wrong Choice", 400)
            }
            
        })

        // get worker-version endpoint useful when testing intially 
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
