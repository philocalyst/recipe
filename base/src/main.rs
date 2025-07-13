#![recursion_limit = "512"]
use std::error::Error;

use cooklang::model::Content;
use cooklang::scale::Scaled;
use cooklang::{self, Converter, CooklangParser, Extensions, Item, Modifiers, Recipe, Step, Value};
use html::content::{Heading1, Heading2, Heading3};
use html::forms::{Form, Option as SelectOption, Select};
use html::inline_text::Span;
use html::interactive::{Details, Summary};
use html::media::Image;
use html::root::Body;
use html::text_content::builders::{DivisionBuilder, ListItemBuilder, OrderedListBuilder};
use html::text_content::{Division as Div, ListItem, OrderedList, Paragraph, UnorderedList};
const TEST_STRING: &str = "---
title: Pecan Coffee Cake
author: Kayla Woods
servings: 16
---

= Prepare Pecans
Preheat the #oven to 350°F (175°C).  
Spread @pecans{135%g}(chopped) on a #sheet pan{} and toast for ~{10%minutes}, or until fragrant. -- Avoid over-toasting to prevent acrid flavor.  
Pulse @pecans{25%g}(roasted) in a #spice grinder{} or #food processor{} until finely ground (about ten pulses). Reserve the remaining roasted pecans for the streusel.

= Make Batter
In a #bowl{}, whisk @flour{250%g}, @baking powder{14%g}, @salt{1.5%g}, and the finely ground pecans.  
In a #mixer{}, cream @butter{225%g} and @sugar{300%g} at medium-low speed for ~{30%seconds}, scraping down the bowl as needed.  
Add @eggs{2} one at a time at low speed, then mix in @vanilla extract{15%ml} and @sour cream{230%g}.  
Scrape down and fold in the flour mixture with a spatula until just combined. -- A few lumps are fine; they’ll cook out.

= Streusel
Melt @butter{30%g} in a #microwave{} for ~{15%seconds}, swirling if needed.  
In a #bowl{}, combine @brown sugar{110%g}, @pecans{110%g}(chopped), and @cinnamon{1%tsp}.  
Pour in the melted butter and stir until evenly moistened.

= Bake & Cool
Prepare a 9×13-inch #baking pan{} by greasing the bottom and sides with butter, then lining with a sheet of parchment paper that extends over the edges to form handles—press it flush to avoid wrinkles.  
Pour the batter into the pan and level with a spatula. Evenly sprinkle the streusel over the top.  
Bake on the middle rack of the #oven at 350°F (175°C) for ~{35%minutes}, or until a toothpick inserted into the center comes out clean.  
Let cool in the pan for ~{10%minutes}, then use the parchment sling to lift the cake onto a #wire rack{} to cool completely.  
> I’ve never managed to let it cool fully—I always cut in too early because I’m too eager for a taste!

= Serve
Once cooled (or if you can’t wait), transfer to a cutting board and cut into sixteen pieces 2×3 inches each.  
Coffee cake is rich and best enjoyed in moderate slices with a cup of coffee or tea—though you can always grab a second piece!
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

    let template = generate_recipe_html(&recipe, &converter)
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
        .replace("1/10", "⅒");

    println!("{}", template);

    Ok(())
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
                    let paragraph = Paragraph::builder().text(text.clone()).build();
                    sect_div.push(paragraph);
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
