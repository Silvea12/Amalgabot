use futures::future::BoxFuture;
use once_cell::sync::Lazy;
use serenity::builder::CreateButton;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::future::Future;
use std::sync::Arc;
use std::sync::Mutex;

// pub struct MenuManager<Fut>
// where
//     Fut: Future<Output = bool>,
// {
//     calls: Mutex<HashMap<String, Vec<Box<fn(Context, Arc<Interaction>) -> Fut>>>>,
// }
//
// static MENU_MANAGER: Lazy<MenuManager> = Lazy::new(|| MenuManager {
//     calls: Mutex::new(HashMap::new()),
// });
//
// impl Debug for MenuManager {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let e = self
//             .calls
//             .lock()
//             .unwrap()
//             .iter()
//             .map(|(k, v)| (k.clone(), format!("{} entries", v.len())))
//             .collect::<HashMap<String, String>>();
//
//         f.debug_struct("MenuManager").field("calls", &e).finish()
//     }
// }
//
// impl MenuManager {
//     pub fn listen(&self, event: String, f: MenuFn) {
//         let mut calls = self.calls.lock().unwrap();
//
//         let calls = if let Some(calls) = calls.get_mut(&event) {
//             calls
//         } else {
//             calls.insert(event.clone(), Vec::new());
//             calls.get_mut(&event).unwrap()
//         };
//
//         calls.push(f);
//     }
//
//     pub async fn call(&self, event: String, ctx: Context, interaction: Interaction) {
//         let mut calls = self.calls.lock().unwrap();
//
//         let calls = if let Some(calls) = calls.get_mut(&event) {
//             calls
//         } else {
//             return;
//         };
//
//         let interaction = Arc::new(interaction);
//         for call in calls.iter_mut() {
//             let mut call = call(ctx.clone(), interaction.clone());
//             Box::pin(call).await;
//         }
//     }
// }

type FnItem = Box<
    dyn Fn(Option<Context>, Option<Arc<Interaction>>) -> BoxFuture<'static, bool> + Send + 'static,
>;

struct Registry {
    e: Vec<FnItem>,
}

static REGISTRY: Lazy<Mutex<Registry>> = Lazy::new(|| Mutex::new(Registry { e: Vec::new() }));

pub fn call() {
    let mut registry = REGISTRY.lock().unwrap();
    let f = registry.e.pop().unwrap();
    let fut = f(None, None);
    let handle = tokio::spawn(async move { fut.await });

    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(handle).unwrap();
    });
}

pub trait BindButton {
    fn bind_fn<F, Fut>(&mut self, f: F) -> &mut Self
    where
        F: Fn(Option<Context>, Option<Arc<Interaction>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = bool> + Send + 'static;
}

impl BindButton for CreateButton {
    fn bind_fn<F, Fut>(&mut self, f: F) -> &mut Self
    where
        F: Fn(Option<Context>, Option<Arc<Interaction>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = bool> + Send + 'static,
    {
        let f: FnItem = Box::new(move |ctx, interaction| Box::pin(f(ctx, interaction)));

        let mut registry = REGISTRY.lock().unwrap();
        registry.e.push(f);

        self
    }
}
