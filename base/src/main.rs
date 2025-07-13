#![recursion_limit = "512"]
use std::error::Error;

use cooklang::{
    self, Converter, CooklangParser, Extensions, Item, Modifiers, Recipe, Step, Value,
    model::Content, scale::Scaled,
};
use html::{
    content::{Heading1, Heading2, Heading3},
    forms::{Form, Input, Option as SelectOption, Select},
    inline_text::Span,
    interactive::{Details, Summary},
    media::Image,
    root::Body,
    text_content::{
        Division as Div, ListItem, OrderedList, Paragraph, UnorderedList,
        builders::{ListItemBuilder, OrderedListBuilder},
    },
};

const TEST_STRING: &str = "---
title: Banana Nut Bread
description: Banana nut breads come in all varieties. Popular recipes result in textures ranging from moist to cakey. Personally, I like banana nut bread that is denser than a classic yellow cake, but not quite as dry as wheat bread. The banana nut bread should be tender and flavorful, but not have the consumer feel like he needs to drink a glass of water with it. Some of the recipes that I've tried were so moist that the bread sticks to the roof of the mouth, while other recipes were much too dry - both require drinking a glass of water to get the bread down. (Of course, drinking a glass of milk while eating a slice of good banana bread is an awesome combination, but it shouldn't be considered a necessity for enjoying banana nut bread.) This recipe results in what I feel is the perfect combination of flavor and texture in banana nut bread.
servings: 10
time: 55 minutes
course: bread
difficulty: moderate
---

> Banana nut bread should always start with fully ripe bananas. Unfortunately, ripe bananas are not usually sold in the supermarket. While a banana ripens, the starch of the banana slowly converts to sugars. Allow green or yellow bananas to ripen at room temperature until the skin is liberally covered with brown spots. Once the banana has reached this stage, it is fully ripe. Bananas can be frozen once they have reached the desired ripeness. Their peels will turn completely brown, but don't worry about the banana within. When ready to use, simply thaw the bananas by letting them sit (unpeeled) on the counter until they warm up. Once thawed, peel the bananas.

Start by preparing a #loaf pan{5in} by buttering the bottom and sides. Lightly flour the pan and tap out the excess flour. The loaf pan should be around 5 in. by 9 in. (13 cm by 23 cm) in size - a little larger or smaller isn't a problem.

> The wet ingredients are: two ripe bananas, 6 Tbs. melted butter, 1 tsp. vanilla extract, and two large eggs. For the dry ingredients: 1-1/3 cup flour, 1/2 tsp. baking soda, 1/4 tsp. baking powder, 2/3 cup sugar, 1/2 tsp. salt, and 1/2 cup chopped walnuts. 

Combine and whisk all the dry ingredients except for the walnuts: @all-purpose flour{1-1/3%cups}, @sugar{2/3%cup}, @baking soda{1/2%tsp}, @baking powder{1/4%tsp}, and @salt{1/2%tsp}. -- The use of both baking soda and powder are necessary to provide enough leavening for the proper consistency of the bread. The baking soda is just enough to utilize the slight acidity of the bananas to create the desired carbon dioxide bubbles. Baking powder (which is a mix of baking soda, a base, and cream of tartar, an acid) provides even more leavening power.

Mash @ripe bananas{2%large}, @melted butter{6%Tbs}, and @vanilla extract{1%tsp} together. Lightly beat @large eggs{2} together.

Mash the banana mixture with the eggs until smooth and well blended.

Pour the banana mixture onto the dry ingredients. Add @chopped walnuts{1/2%cup}.

Fold the ingredients together until no more white flour is uncovered while folding.

Pour the batter into the prepared #loaf pan{} and bake for ~{55%minutes} at 350°F.

After ~{55%minutes}, the loaf of banana bread should be done. A wooden toothpick inserted into the center should come out clean. Set the pan on a #wire rack{} to cool for ~{10%minutes}.

Remove the loaf from the pan and let cool on the #wire rack{}. Serve warm or fully cooled. The loaf can be wrapped in plastic and stored at room temperature for about four or five days.
";

const DEFAULT_SERVINGS: u32 = 2;

fn main() -> Result<(), Box<dyn Error>> {
    let parser = CooklangParser::new(Extensions::all(), Converter::default());

    let (recipe, _warnings) = parser.parse(TEST_STRING).into_result()?;

    // We're using the units that the recipe defines
    let converter = Converter::builder().with_bundled_units()?.finish()?;

    // Retrieve the default system of the recipe
    // let current_system = converter.default_system();

    let recipe = recipe.scale_to_servings(DEFAULT_SERVINGS, &converter);

    let template = fractions_to_unicode(generate_recipe_html(&recipe, &converter));

    println!("{}", template);

    Ok(())
}

fn fractions_to_unicode(input: String) -> String {
    input
        .replace("1/2", "½")
        .replace("1/3", "⅓")
        .replace("2/3", "⅔")
        .replace("1/4", "¼")
        .replace("3/4", "¾")
        .replace("1/5", "⅕")
        .replace("2/5", "⅖")
        .replace("3/5", "⅗")
        .replace("4/5", "⅘")
        .replace("1/6", "⅙")
        .replace("5/6", "⅚")
        .replace("1/7", "⅐")
        .replace("1/8", "⅛")
        .replace("3/8", "⅜")
        .replace("5/8", "⅝")
        .replace("7/8", "⅞")
        .replace("1/9", "⅑")
}

/// Function to generate the default recipe page
fn generate_recipe_html(recipe: &Recipe<Scaled, Value>, converter: &Converter) -> String {
    // Generation of the <body> element that wraps the data
    let mut body = Body::builder();
    body.data("system", converter.default_system().to_string());

    // The hero/header image of the recipe
    if let Some(main_image) =
        Some("/Users/philocalyst/Projects/recipe/food/AsparagusSoup/asparagus-soup.jpg".to_string())
    {
        let img_div = Div::builder()
            .push(Image::builder().src(main_image).build())
            .build();
        body.push(img_div);
    }

    let meta = &recipe.metadata;
    let cookwares = &recipe.cookware;

    // The heading/author/servings container
    let mut main_recipe_info = Div::builder();
    main_recipe_info.class("main-info");

    // The title of the recipe
    let mut h1_builder = Heading1::builder();

    match meta.title() {
        Some(title) => {
            h1_builder.push(title.to_owned());
        }
        None => {
            println!("No title found!");
        }
    }

    main_recipe_info.push(h1_builder.build());

    // Author/source group.
    if meta.author().is_some() || meta.source().is_some() {
        let name_and_url = meta
            .author()
            .unwrap_or_else(|| meta.source().expect("Exists!!"));

        let mut author = match name_and_url.name() {
            Some(name) => name.to_owned(),
            None => "Unknown".into(),
        };

        // If it's somehow a URL, just assume the name is unknown
        if author.contains(".") {
            author = "Unknown".into();
        }

        let author_div = Div::builder()
            .push(Div::builder().push(Span::builder().build()).build())
            .text(format!("By {}", author))
            .build();
        main_recipe_info.push(author_div);
    }

    // Servings group.

    let provided_servings = meta.servings().unwrap_or(vec![1])[0];

    main_recipe_info.push(create_servings_form(provided_servings));

    body.push(main_recipe_info.build());

    // Other recipe information
    let mut tags_div = Div::builder();
    tags_div.class("tags");
    if let Some(tags) = meta.tags() {
        for tag in tags {
            tags_div.push(Div::builder().class("tag").text(tag.to_string()).build());
        }
    }
    body.push(tags_div.build());

    // The description of the recipe
    if let Some(desc) = meta.description() {
        let p = Paragraph::builder().text(desc.to_string()).build();
        body.push(p);
    }

    // Time group.
    if let Some(time) = &recipe.metadata.time(converter) {
        let time_text = format!("{} minutes", time.total());
        let time_div = Div::builder()
            .push(Div::builder().push(Span::builder().build()).build())
            .push(Div::builder().text(time_text).build())
            .build();
        body.push(time_div);
    }

    // More metadata details.
    let more_details = Details::builder()
        .id("recipe-more-metadata")
        .push(Summary::builder().push("More Data").build())
        .push(Div::builder().push("Added: 2023-01-01").build())
        .build();
    body.push(more_details);

    // Form wrapper for the select element
    let mut form = Form::builder();

    // Select dropdown for unit selection
    let mut select = Select::builder();
    select.id("units").name("units");

    // We only support metric and imperial
    for sys in ["metric", "imperial"] {
        let option_text = sys;

        if converter.default_system().to_string() == sys {
            select.push(
                SelectOption::builder()
                    .value(sys)
                    .selected(true)
                    .text(option_text)
                    .build(),
            );
        } else {
            select.push(SelectOption::builder().value(sys).text(option_text).build());
        }
    }
    form.push(select.build());

    // Wrapper for the general recipes content
    let mut recipe_content = Div::builder();
    recipe_content.data("igr-layout", "line");

    // The top-level ingridients and cookware info
    let mut recipe_logistics = Div::builder();
    recipe_logistics.class("logistics");

    let mut ingredients_container = Div::builder();
    let mut cookware_container = Div::builder();

    let mut ingredients_list = UnorderedList::builder();
    for (_, ingredient) in recipe.ingredients.iter().enumerate() {
        let mut list_item = ListItem::builder();
        list_item.push(
            Span::builder()
                .text(match &ingredient.note {
                    Some(note) => format!("{} ({})", ingredient.name, note),
                    None => ingredient.name.clone(),
                })
                .build(),
        );
        if ingredient.modifiers().contains(Modifiers::OPT) {
            let opt_text = format!(" ({})", "OptMarker");
            list_item.text(opt_text);
        }
        if let Some(qty) = &ingredient.quantity {
            let qty_text = format!(": {}", qty);

            let ing_span = Span::builder()
                .data("source-units", qty.to_string())
                .class("quantity")
                .text(qty_text)
                .build();

            list_item.push(ing_span);
        }
        ingredients_list.push(list_item.build());
    }

    // Setup
    let mut ingredients_header = Div::builder();
    ingredients_header.class("ingredient_header");
    ingredients_header.push(Heading2::builder().text("Ingredients").build());
    ingredients_header.push(form.build());

    ingredients_container.push(ingredients_header.build());
    ingredients_container.push(ingredients_list.build());

    let mut cookware_ul = UnorderedList::builder();
    for (index, cookware) in cookwares.iter().enumerate() {
        let mut li = ListItem::builder();
        li.push(
            Span::builder()
                .text(cookware.display_name().to_string())
                .build(),
        );
        cookware_ul.push(li.build());
    }

    if recipe.cookware.len() != 0 {
        // Setup
        cookware_container.push(Heading2::builder().push("Cookware").build());
        cookware_container.push(cookware_ul.build());
    }

    // Assign both
    recipe_logistics.push(ingredients_container.build());
    recipe_logistics.push(cookware_container.build());

    // Assign to the big mom
    recipe_content.push(recipe_logistics.build());

    // The actual instructions

    if recipe.sections.len() < 2 {
        recipe_content.push(Heading2::builder().text("Steps").build());
    } else {
        recipe_content.push(Heading2::builder().text("Method").build());
    }

    for (sect_index, sect) in recipe.sections.iter().enumerate() {
        let section_id = format!("section-{}", sect_index);

        let section_index_str = sect_index.to_string();
        let section_name = sect.name.clone().unwrap_or_else(|| "Section".into());

        let mut sect_div = Div::builder();
        sect_div
            .data("section-index", section_index_str)
            .id(section_id);

        if recipe.sections.len() > 1 {
            sect_div.push(Heading3::builder().text(section_name).build());
        }

        let mut list_div = OrderedList::builder();

        for content in &sect.content {
            match content {
                Content::Step(step) => {
                    add_step_to_list(&mut list_div, step, &sect.content, recipe);
                }
                Content::Text(text) => {
                    list_div.list_item(|li| li.text(text.to_owned()).role("note"));
                }
            }
        }

        sect_div.push(list_div.build());

        recipe_content.push(sect_div.build());
    }

    body.push(recipe_content.build());

    // <script> at the end.
    body.push(
        html::scripting::Script::builder()
            .src("/js/recipe.js")
            .data("defer", "")
            .build(),
    );

    // Build and render to string.
    format!(
        "{}<style>{}</style>",
        body.build(),
        include_str!("../style.css")
    )
}

// Handles adding a step as a list item
fn add_step_to_list(
    list_div: &mut OrderedListBuilder,
    step: &Step,
    all_content: &[Content],
    recipe: &Recipe<Scaled, Value>,
) {
    list_div.list_item(|li| {
        if all_content.len() == 1 {
            li.class("sole");
        }

        for item in &step.items {
            add_item_to_list_item(li, item, recipe);
        }

        li
    });
}

// Processes individual items within a step
fn add_item_to_list_item(li: &mut ListItemBuilder, item: &Item, recipe: &Recipe<Scaled, Value>) {
    match item {
        Item::Text { value } => {
            li.text(value.clone());
        }
        Item::Ingredient { index } => {
            add_ingredient_span(li, *index, recipe);
        }
        Item::Cookware { index } => {
            add_cookware_span(li, *index, recipe);
        }
        Item::Timer { index } => {
            add_timer_span(li, *index, recipe);
        }
        Item::InlineQuantity { index } => {
            add_quantity_span(li, *index, recipe);
        }
    }
}

// Creates and adds an ingredient span
fn add_ingredient_span(li: &mut ListItemBuilder, index: usize, recipe: &Recipe<Scaled, Value>) {
    let ingredient = &recipe.ingredients[index];
    let span = Span::builder()
        .text(ingredient.display_name().to_string())
        .class("ingredient")
        .build();
    li.push(span);
}

// Creates and adds a cookware span
fn add_cookware_span(li: &mut ListItemBuilder, index: usize, recipe: &Recipe<Scaled, Value>) {
    let cookware = &recipe.cookware[index];
    let span = Span::builder()
        .text(cookware.display_name().to_string())
        .class("cookware")
        .build();
    li.push(span);
}

// Creates and adds a timer span
fn add_timer_span(li: &mut ListItemBuilder, index: usize, recipe: &Recipe<Scaled, Value>) {
    let timer = &recipe.timers[index];
    let timer_text = timer.quantity.clone().unwrap().to_string();

    let span = Span::builder().text(timer_text).class("timer").build();
    li.push(span);
}

// Creates and adds a quantity span
fn add_quantity_span(li: &mut ListItemBuilder, index: usize, recipe: &Recipe<Scaled, Value>) {
    let quantity = &recipe.inline_quantities[index];
    let quantity_text = quantity.to_string();

    let span = Span::builder()
        .text(quantity_text.clone())
        .class("quantity")
        .data("metric", quantity_text)
        .build();
    li.push(span);
}

fn create_servings_form(servings: u32) -> Form {
    Form::builder()
        .text("makes")
        .push(
            Input::builder()
                .value(servings.to_string())
                .type_("number")
                .min("1")
                .max("99")
                .input_mode("numeric")
                .pattern("[0-9]*")
                .build(),
        )
        .build()
}
