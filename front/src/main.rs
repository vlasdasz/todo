#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(arbitrary_self_types)]

use test_engine::App;

use crate::todo_view::TodoView;

mod todo_view;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    App::start::<TodoView>().await
}
