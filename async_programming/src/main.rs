async fn hello_world() {
    println!("Hello, world!");
}

enum AsyncCalls {
    GET,
    POST
}

struct AsyncDemo {
    url: &'static str
}

impl AsyncDemo {
    async fn async_call(&self, endpoint: &str, method: AsyncCalls, post_data: Option<&'static str>) {
        match method {
            AsyncCalls::GET => {
                self.get_request(endpoint).await
            },
            AsyncCalls::POST => {
                self.post_request(endpoint, post_data.unwrap()).await
            }
        }
    }

    async fn get_request(&self, endpoint: &str) {
        let req_url = format!("{}/{}", self.url, endpoint);
        let body = reqwest::get(req_url).await.unwrap();
        let text = body.text().await.unwrap();

        println!("GET Response = {text}");
    }

    async fn post_request(&self, endpoint: &str, data: &'static str) {
        let req_url = format!("{}/{}", self.url, endpoint);
        let client = reqwest::Client::new();
        let res = client.post(req_url).body(data).send().await.unwrap();
    
        println!("POST Response = {res:?}");
    }
}

#[tokio::main]
async fn main() {
    hello_world().await;

    let async_demo = AsyncDemo {
        url: "https://jsonplaceholder.typicode.com"
    };

    async_demo.async_call("todos/1", AsyncCalls::GET, None).await;
    async_demo.async_call("posts", AsyncCalls::POST, Some("my-data")).await;
}