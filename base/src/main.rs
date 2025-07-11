#![recursion_limit = "512"]
use std::error::Error;

use cooklang::model::Content;
use cooklang::scale::Scaled;
use cooklang::{
    self, Converter, CooklangParser, Extensions, Item, Modifiers, Recipe, Value, parser,
};
const TEST_STRING: &str = "---
source: www.kookstudio75.nl
servings: 4
course: Did someone move my cheese
title: Crispy apple beet root
description: Beet root made of Granny Smith apple covered with beet root powder
---

Mix @granny smith apple juice{250%ml} with @protein_powder{11%g} and @xanthan_gum{10%mg} with a #blender.

Cover it and let it sit for ~{2%hours} in the #fridge.

Beat the mass with a #planet mixer{} (e.g. KitchenAid) with a #balloon whisk{} to a strong meringue.

Transfer the meringue into a #piping bag{} with a #smooth nozzle{}.

Make small meringues onto the #baking sheet{} in the shape of a beet root.

Sprinkle with @beet root powder{} (buy at a biological grocery store) and let dry in the #oven at 70C for ~{2%hours}.
";

const DEFAULT_SERVINGS: u32 = 2;

fn main() -> Result<(), Box<dyn Error>> {
    let parser = CooklangParser::new(Extensions::all(), Converter::default());

    let (recipe, _warnings) = parser.parse(TEST_STRING).into_result()?;

    // We're using the units that the recipe defines
    let converter = Converter::builder().with_bundled_units()?.finish()?;

    // Retrieve the default system of the recipe
    let current_system = converter.default_system();

    let recipe = recipe.scale_to_servings(DEFAULT_SERVINGS, &converter);

    let section = recipe.sections[0].clone();

    let template = generate_recipe_html(&recipe, &converter);

    println!("{}", template);

    Ok(())
}

use html::content::{Heading1, Heading2};
use html::embedded::Iframe;
use html::forms::Button;
use html::forms::{Form, Input, Option as SelectOption, Select};
use html::inline_text::{Anchor, Span};
use html::interactive::{Details, Summary};
use html::media::Image;
use html::metadata::Link; // For links, but used as Anchor here.
use html::root::Body;
use html::text_content::{Division as Div, ListItem, OrderedList, Paragraph, UnorderedList};
use std::collections::HashMap;

// Dummy translation function.
fn t(key: &str) -> String {
    match key {
        "r.warnings" => "Warnings".to_string(),
        "r.meta.servings" => "Servings".to_string(),
        "r.meta.author" => "Author".to_string(),
        "r.meta.source" => "Source".to_string(),
        "r.meta.totalTime" => "Total Time".to_string(),
        "r.meta.prepTime" => "Prep Time".to_string(),
        "r.meta.cookTime" => "Cook Time".to_string(),
        "r.meta.moreData" => "More Data".to_string(),
        "r.meta.added" => "Added".to_string(),
        "r.meta.modified" => "Modified".to_string(),
        "r.meta.sourceFile" => "Source File".to_string(),
        "r.convertSelector.label" => "Units".to_string(),
        "r.convertSelector.default" => "Default".to_string(),
        "r.convertSelector.metric" => "Metric".to_string(),
        "r.convertSelector.imperial" => "Imperial".to_string(),
        "r.ingredients" => "Ingredients".to_string(),
        "r.cookware" => "Cookware".to_string(),
        "r.optMarker" => "optional".to_string(),
        "r.method" => "Method".to_string(),
        "r.section" => "Section".to_string(),
        "r.ref.fromStep" => "from step".to_string(),
        "r.ref.fromSect" => "from section".to_string(),
        "timer.start" => "Start Timer".to_string(),
        // Add more as needed...
        _ => key.to_string(),
    }
}

// Function to generate the HTML string using the html crate.
// Function to generate the HTML string using the html crate.
fn generate_recipe_html(r: &Recipe<Scaled, Value>, converter: &Converter) -> String {
    let mut body = Body::builder();

    // <!-- Image(s) -->
    if let Some(main_image) = Some("/path/to/image.jpg".to_string()) {
        // Assuming some image.
        let img_div = Div::builder()
            .class("mb-8 max-h-[70vh] w-full overflow-hidden rounded shadow-lg")
            .push(
                Image::builder()
                    .class("h-full w-full object-cover")
                    .src(main_image)
                    .build(),
            )
            .build();
        body.push(img_div);
    }

    // <!-- Warnings -->
    if true {
        // Assuming report_html exists.
        let details = Details::builder()
            .id("recipe-warnings")
            .data("remember-open", "") // Custom data.
            .push(
                Summary::builder()
                    .class("font-bold text-yellow-11")
                    .push(t("r.warnings"))
                    .build(),
            )
            .push(Div::builder().push("Sample warning content.").build()) // Placeholder for include.
            .build();
        body.push(details);
    }

    let mut h1_builder = Heading1::builder();
    h1_builder.push("Pancakes");
    h1_builder.class("font-heading text-6xl");
    body.push(h1_builder.build());

    // <!-- Metadata -->
    let mut meta_div = Div::builder(); // Store the builder itself
    meta_div.class("m-4 flex flex-wrap gap-2");
    for tag in &r.metadata.tags() {
        // Assuming tag macro as a span.
        meta_div.push(Span::builder().text(tag.join(" ")).build()); // Clone the tag name
    }
    body.push(meta_div.build());

    if let Some(desc) = &r.metadata.description() {
        let p = Paragraph::builder()
        .class("m-4 w-fit text-balance rounded border-l-4 border-primary-9 bg-base-2 p-4 text-xl shadow")
        .text(desc.to_string()) // Convert to owned String
        .build();
        body.push(p);
    }

    // Meta groups (simplified, skipping macros for brevity).
    // Servings group.
    let servings_div = Div::builder()
        .class("my-3 flex w-fit flex-wrap items-center justify-start gap-2 rounded-xl border border-dashed border-base-6 p-2")
        .push(Div::builder().class("...").push(Span::builder().class("i-lucide-utensils").build()).build()) // Icon placeholder.
        .push(Div::builder().push(t("r.meta.servings")).build()) // Simplified entry.
        .build();
    body.push(servings_div);

    // Author/source group.
    if r.metadata.author().is_some() || r.metadata.source().is_some() {
        let author_div = Div::builder()
            .class("my-3 flex w-fit flex-wrap items-center justify-start gap-2 rounded-xl border border-dashed border-base-6 p-2")
            .push(Div::builder().class("...").push(Span::builder().class("i-lucide-user").build()).build())
            .push(Div::builder().push(t("r.meta.author")).build()) // Simplified.
            .build();
        body.push(author_div);
    }

    // Time group.
    if let Some(time) = &r.metadata.time(converter) {
        let time_text = format!("{} minutes", time.total());
        let time_div = Div::builder()
            .class("my-3 flex w-fit flex-wrap items-center justify-start gap-2 rounded-xl border border-dashed border-base-6 p-2")
            .push(Div::builder().class("...").push(Span::builder().class("i-lucide-hourglass").build()).build())
            .push(Div::builder().text(time_text).build()) // Use owned string
            .build();
        body.push(time_div);
    }

    // More metadata details.
    let more_details = Details::builder()
        .id("recipe-more-metadata")
        .push(
            Summary::builder()
                .class("w-fit text-primary-12")
                .push(t("r.meta.moreData"))
                .build(),
        )
        .push(Div::builder().push("Added: 2023-01-01").build()) // Placeholder.
        .build();
    body.push(more_details);

    // <!-- Controls -->
    let mut form = Form::builder(); // Store the builder itself
    form.class("float-right my-2")
        .method("get")
        .action("/some/path")
        .data("hx-trigger", "input changed from:#units, submit")
        .data("hx-swap", "show:none");

    // Add select for units.
    let mut select = Select::builder(); // Store the builder itself
    select
        .id("units")
        .name("units")
        .class("rounded border border-base-4 bg-base-2 px-1 py-0.5");

    for sys in ["default", "metric", "imperial"] {
        let option_text = t(&format!("r.convertSelector.{}", sys));
        select.push(
            SelectOption::builder()
                .value(sys)
                .text(option_text) // Use owned string from t()
                .build(),
        );
    }
    form.push(select.build());
    body.push(form.build());

    // <!-- Recipe content -->
    let mut content_div = Div::builder(); // Store the builder itself
    content_div
        .class("content font-serif text-lg")
        .data("data-igr-layout", "line");

    // Ingredients list.
    let mut ingredients_ul = UnorderedList::builder(); // Store the builder itself
    ingredients_ul.class("ms-6 list-disc");
    for (i, ing) in r.ingredients.iter().enumerate() {
        let mut li = ListItem::builder();
        li.push(
            Span::builder()
                .data("data-component-kind", "ingredient")
                .data("data-component-ref-group", i.to_string()) // Use owned string
                .data("data-component-ref-target", "ingredient")
                .text(ing.name.clone()) // Clone the display name
                .build(),
        );
        if ing.modifiers().contains(Modifiers::OPT) {
            let opt_text = format!(" ({})", t("r.optMarker"));
            li.text(opt_text); // Use owned string
        }
        if let Some(qty) = &ing.quantity {
            let qty_text = format!(": {}", qty);
            li.text(qty_text); // Use owned string
        }
        ingredients_ul.push(li.build());
    }
    content_div.push(
        Heading2::builder()
            .class("my-2 font-heading text-3xl")
            .text(t("r.ingredients"))
            .build(),
    );
    content_div.push(ingredients_ul.build());

    // Cookware list (similar).
    let mut cookware_ul = UnorderedList::builder(); // Store the builder itself
    cookware_ul.class("ms-6 list-disc");
    for (i, cw) in r.cookware.iter().enumerate() {
        let mut li = ListItem::builder();
        li.push(
            Span::builder()
                .data("data-component-kind", "cookware")
                .data("data-component-ref-group", i.to_string()) // Use owned string
                .data("data-component-ref-target", "cookware")
                .text(cw.display_name().to_string()) // Clone the display name
                .build(),
        );
        cookware_ul.push(li.build());
    }
    content_div.push(
        Heading2::builder()
            .class("my-2 font-heading text-3xl")
            .push(t("r.cookware"))
            .build(),
    );
    content_div.push(cookware_ul.build());

    // Sections / Method.
    content_div.push(
        Heading2::builder()
            .class("mb-2 mt-6 font-heading text-3xl")
            .push(t("r.method"))
            .build(),
    );
    for (sect_index, sect) in r.sections.iter().enumerate() {
        let section_id = format!("section-{}", sect_index);
        let section_index_str = sect_index.to_string();
        let section_name = sect
            .name
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or_else(|| t("r.section"));

        let mut sect_div = Div::builder(); // Store the builder itself
        sect_div
            .class("bg-transparent transition-colors")
            .data("data-section-index", section_index_str) // Use owned string
            .id(section_id); // Use owned string

        sect_div.push(
            Heading2::builder()
                .class("my-3 font-heading text-2xl")
                .text(section_name) // Use owned string
                .build(),
        );

        let mut list_div = OrderedList::builder();

        for content in &sect.content {
            match content {
                Content::Step(step) => {
                    list_div.list_item(|li| {
                        li.class("my-6 flex").class("grow");

                        for item in &step.items {
                            // Use reference to avoid clone
                            match item {
                                Item::Text { value } => {
                                    li.text(value.clone());
                                }
                                Item::Ingredient { index } => {
                                    li.text(
                                        Span::builder()
                                            .class("font-semibold text-green-11")
                                            .text(r.ingredients[*index].display_name().to_string())
                                            .build()
                                            .to_string(),
                                    );
                                }
                                Item::Cookware { index } => {
                                    li.text(
                                        Span::builder()
                                            .class("font-semibold text-green-11")
                                            .text(r.cookware[*index].display_name().to_string())
                                            .build()
                                            .to_string(),
                                    );
                                }
                                Item::Timer { index } => {
                                    li.text(
                                        Span::builder()
                                            .class("font-semibold text-green-11")
                                            .text(
                                                r.timers[*index]
                                                    .quantity
                                                    .clone()
                                                    .unwrap()
                                                    .to_string(),
                                            )
                                            .build()
                                            .to_string(),
                                    );
                                }
                                Item::InlineQuantity { index } => {
                                    li.text(
                                        Span::builder()
                                            .class("font-semibold text-green-11")
                                            .text(r.inline_quantities[*index].to_string())
                                            .build()
                                            .to_string(),
                                    );
                                }
                            }
                        }

                        li
                    });
                }
                Content::Text(text) => {
                    sect_div.push(
                        Paragraph::builder()
                            .class("indent-4")
                            .text(text.clone())
                            .build(),
                    ); // Clone the text
                }
            }
        }

        sect_div.push(list_div.build());

        content_div.push(sect_div.build());
    }

    body.push(content_div.build());

    // <script> at the end.
    body.push(
        html::scripting::Script::builder()
            .src("/js/recipe.js")
            .data("defer", "")
            .build(),
    );

    // Build and render to string.
    body.build().to_string()
}
