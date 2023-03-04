use yew::{html, Html};

use crate::components::layout::Layout;

/// Arguments
/// * `title` - The title of the card
/// * `image` - The path to the image
fn trending_recipes_card(recipe: TrendingRecipe) -> Html {
    html! {
    <div class="relative bg-orange-100 border rounded-lg shadow-md dark:bg-gray-800 dark:border-gray-700 transform transition duration-500 hover:scale-105">
        <div class="p-2 flex justify-center">
            <a href="#">
                <img class="rounded-md object-cover block h-72 w-full" src={recipe.image} loading="lazy" />
            </a>
        </div>


        <div class="px-4 pb-3">

            <div>
                <a href="#">
                    <h5
                        class="text-xl font-semibold tracking-tight hover:text-orange-800 dark:hover:text-orange-400 text-gray-900 dark:text-white ">
                        {recipe.title}
                    </h5>
                </a>

                <p class="antialiased text-gray-600 dark:text-gray-400 text-sm break-all truncate md:overflow-clip">
                    <b>{"Ingredients: "}</b> {recipe.ingredients.join(", ").chars().take(50).collect::<String>()}
                </p>

            </div>

        </div>

    </div>
    }
}

#[derive(Debug, Clone)]
struct TrendingRecipe {
    title: &'static str,
    image: &'static str,
    ingredients: Vec<String>,
}

pub fn page() -> Html {
    let trending_recipes = vec![
        TrendingRecipe {
            title: "Salted fish with Rice",
            image: "images/food/salted-fish-with-rice.png",
            ingredients: vec![
                "white rice".to_string(),
                "salted fish".to_string(),
                "onions".to_string(),
                "seasoning peppers".to_string(),
            ],
        },
        TrendingRecipe {
            title: "One Bowl Chocolate Cake",
            image: "images/food/chocolate-cake.png",
            ingredients: vec![],
        },
        TrendingRecipe {
            title: "Pelau",
            image: "images/food/pelau.png",
            ingredients: vec![],
        },
    ];

    html! {
       <Layout>
            <>
                <section class="dark:text-gray-100">
                   <div class="container flex flex-col justify-center p-6 mx-auto sm:py-12 lg:py-24 lg:flex-row lg:justify-between">
                      <div class="flex flex-col justify-center p-6 text-center rounded-sm lg:max-w-md xl:max-w-lg lg:text-left">
                         <h1 class="text-5xl text-orange-500 font-bold leading-none sm:text-5xl">
                             {"Delicious Recipes"}
                         </h1>

                         <br/>

                         <h1 class="text-6xl font-bold leading-none sm:text-5xl">
                             {"For Food Lovers!"}
                         </h1>

                         <div class="flex flex-col mt-2 space-y-4 sm:items-center sm:justify-center sm:flex-row sm:space-y-0 sm:space-x-4 md:mt-8 sm:mt-4 lg:justify-start">
                                <a rel="noopener noreferrer" href="#" class="px-8 py-3 text-lg font-semibold bg-orange-300 border rounded dark:border-gray-100 dark:bg-orange-500 dark:border-none">{"Explore"}</a>
                         </div>
                      </div>

                      <div class="flex items-center justify-center p-6 mt-8 lg:mt-0 h-72 sm:h-80 lg:h-96 xl:h-112 2xl:h-128">
                         <img src="images/food/salted-fish-with-rice.png" alt="" class="object-contain h-72 sm:h-80 lg:h-96 xl:h-112 2xl:h-128 drop-shadow-2xl"/>
                      </div>
                   </div>
                </section>

                <section class="container mx-auto px-6 p-10 dark:text-gray-100">
                    <h2 class="text-4xl font-bold text-left mb-8">
                        {"Trending "}<span class="text-orange-500">{"Recipes"}</span>
                    </h2>

                    <div class="grid gap-8 grid-cols-1 md:grid-cols-3 p-4 md:p-2 xl:p-5">

                    {
                        trending_recipes.into_iter().map(|recipe| {
                            trending_recipes_card(recipe)
                        }).collect::<Html>()
                    }

                    </div>


                </section>
            </>
       </Layout>
    }
}
