use yew::{html, Html};

use crate::components::layout::Layout;

pub fn page() -> Html {
    html! {
       <Layout>
            <h1>{"Home Page"}</h1>
       </Layout>
    }
}

