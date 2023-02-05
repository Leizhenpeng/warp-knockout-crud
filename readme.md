<p align='center'>
  <img src='./img.png' alt='Rate The KnockOut Actors with Warp' width='1800'/>
</p>

<p align='center'>
Rate The KnockOut Actors With  <b>Warp</b><sup><em>(crud)</em></sup><br>
</p>


## 介绍

不要紧张，这只是一个简单的CRUD，但是它可以帮助你理解如何使用rust语言和Warp路由来构建restful风格的后端API。

## 关于rust写后端


打个比方： 

凌晨，某集团军钢刀连三排五班接到了一项紧急任务：把一份神秘的物资送往10公里外的前线哨所。

时间紧迫，任务重要，整装待发的五人开上装甲车，立刻出发护送物资。

然而，敌人早已经知道了他们的计划，在路上埋伏了大批武装力量。钢刀连面临着极大的威胁，但他们并未退缩。在一番激烈的战斗中，他们成功护送了物资到达哨所。

暮色将至，哨所长官打开了包裹，却发现里面竟然是一份后勤特供早餐！

看着这份绝密“物资”，班长的两眼发红了。


## Feature

- [x] 用Rust编写
- [x] 使用Warp构建
- [x] 基于Tokio的异步
- [x] 日志记录
- [x] 优雅的路由封装
- [x] 路由自测

## develop

```bash
# first clone the repo
git clone git@github.com:Leizhenpeng/warp-knockout-crud.git
# then cd into the repo,
cd warp-knockout-crud
# install the dependencies
make install

# run the project
make dev
```


## Feature

### 路由自测, 继续保持可爱的习惯

``` rust
mod test {
    use warp::http::StatusCode;
    use super::*;
    use warp::test::request;

    #[tokio::test]
    async fn test_ping_checker() {
        let api = router::load_router(db::init_db());
        let api = api.with(add_cors());
        let routes = api.with(warp::log("api"));
        let resp = request()
            .method("GET")
            .path("/api/ping")
            .reply(&routes)
            .await;
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.body(), r#"{"status":"200","message":"pong"}"#,
        )
    }

    #[tokio::test]
    async fn test_create_checker() {
        let api = router::load_router(db::init_db());
        let api = api.with(add_cors());
        let routes = api.with(warp::log("api"));
        let resp = request()
            .method("POST")
            .path("/api/new")
            .json(&model::CreateActorReq {
                name: "高启强".to_string(),
                description: Some("高启强".to_string()),
                score: 10,
            })
            .reply(&routes)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }
}
```

### 返回体结构,整整齐齐

``` rust
#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct ActorData {
    pub actor: Actor,
}

#[derive(Serialize, Debug)]
pub struct SingleActorResponse {
    pub status: String,
    pub data: ActorData,
}

#[derive(Serialize, Debug)]
pub struct ActorListResponse {
    pub status: String,
    pub results: usize,
    pub actors: Vec<Actor>,
}
```


```json
{
    "status": "success",
    "results": 1,
    "actors": [
        {
            "id": "CFZr",
            "name": "徐江",
            "description": "财大气粗，嚣张跋扈的著名黑社会企业家",
            "score": 7,
            "created_at": "2023-02-05T05:14:44.478703Z",
            "updated_at": "2023-02-05T05:14:44.478703Z"
        }
    ]
}
```


### 日志管理

- 日志美化
- 输出文件保存

``` rust
#[tokio::main]
async fn main() {
    init_log();
    ... 
    let routes = api.with(warp::log("app"));
    ...
}

fn init_log() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "app=info");
    }
    pretty_env_logger::init();
}
```

``` rust
deploy:
	cargo run ./target/debug/warp-knockout-crud 2>&1 | tee test.log

```

``` log
 INFO  warp::server > Server::run; addr=127.0.0.1:3030
 INFO  warp::server > listening on http://127.0.0.1:3030
 DEBUG hyper::proto::h1::io > parsed 6 headers
 DEBUG hyper::proto::h1::conn > incoming body is empty
 INFO  api                    > 127.0.0.1:57862 "GET /api/ping HTTP/1.1" 200 "-" "PostmanRuntime/7.30.1" 171.166µs
 DEBUG hyper::proto::h1::io   > flushed 141 bytes
 ```

### cors support

``` rust
fn add_cors() -> Builder {
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);
    cors
}
```

### 规范项目结构,优雅封装路由

    
```angular2html
.
├── Cargo.lock
├── Cargo.toml
├── Makefile
├── readme.md
├── src
│   ├── db.rs
│   ├── handler.rs
│   ├── main.rs
│   ├── model.rs
│   └── router.rs
└── test.log
```

``` rust
use warp::Filter;
use crate::db::{DB, with_db};
use crate::handler;

pub fn load_router(_db: DB) -> impl Filter<Extract=impl warp::Reply, Error=
warp::Rejection> + Clone {
    let api_base = warp::path("api");
    let api = api_base.and(warp::get()).and(warp::path("ping")).
        and_then(handler::ping_handler);

    //create
    let api_new = api_base.and(warp::post()).
        and(warp::path("new")).
        and(warp::body::json()).
        and(with_db(_db.clone())).
        and_then(handler::actor_create_handler);

    //read
    let api_read = api_base.and(warp::get()).
        and(warp::body::json()).
        and(with_db(_db.clone())).
        and_then(handler::actor_list_handler);

    let api_update = api_base.and(warp::patch()).
        and(warp::path::param()).
        and(warp::body::json()).
        and(with_db(_db.clone())).
        and_then(handler::actor_update_handler);


    let api_delete = api_base.and(warp::delete()).
        and(warp::path::param()).
        and(with_db(_db.clone())).
        and_then(handler::actor_delete_handler);

    api.or(api_new).or(api_read).or(api_update).or(api_delete)
}
```


## More Info

[pretty env logger](https://github.com/seanmonstar/pretty-env-logger)

[nanoid](https://crates.io/crates/nanoid)

[warp](https://github.com/seanmonstar/warp)

[warp-example](https://github.com/seanmonstar/warp/tree/master/examples)
