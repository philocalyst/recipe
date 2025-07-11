#![recursion_limit = "512"]
use std::error::Error;

use cooklang::model::Content;
use cooklang::scale::Scaled;
use cooklang::{
    self, Converter, CooklangParser, Extensions, Item, Modifiers, Recipe, Value, parser,
};
const TEST_STRING: &str = "---
title: Pecan Coffee Cake
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
    let current_system = converter.default_system();

    let recipe = recipe.scale_to_servings(DEFAULT_SERVINGS, &converter);

    let section = recipe.sections[0].clone();

    let template = generate_recipe_html(&recipe, &converter);

    println!("{}", template);

    Ok(())
}

use html::content::{Heading1, Heading2, Heading3};
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

// Function to generate the HTML string using the html crate.
// Function to generate the HTML string using the html crate.
fn generate_recipe_html(r: &Recipe<Scaled, Value>, converter: &Converter) -> String {
    let mut body = Body::builder();

    body.data("system", converter.default_system().to_string());

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
                    .push("Warnings")
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
        .push(Div::builder().push("Servings").build()) // Simplified entry.
        .build();
    body.push(servings_div);

    // Author/source group.
    if r.metadata.author().is_some() || r.metadata.source().is_some() {
        let author_div = Div::builder()
            .class("my-3 flex w-fit flex-wrap items-center justify-start gap-2 rounded-xl border border-dashed border-base-6 p-2")
            .push(Div::builder().class("...").push(Span::builder().class("i-lucide-user").build()).build())
            .push(Div::builder().push("Author").build()) // Simplified.
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
                .push("More Data")
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

    for sys in ["metric", "imperial"] {
        let option_text = sys;

        if converter.default_system().to_string() == sys {
            select.push(
                SelectOption::builder()
                    .value(sys)
                    .selected(true)
                    .text(option_text) // Use owned string from t()
                    .build(),
            );
        } else {
            select.push(
                SelectOption::builder()
                    .value(sys)
                    .text(option_text) // Use owned string from t()
                    .build(),
            );
        }
    }
    form.push(select.build());
    body.push(form.build());

    // <!-- Recipe content -->
    let mut content_div = Div::builder(); // Store the builder itself
    content_div
        .class("content font-serif text-lg")
        .data("igr-layout", "line");

    // Ingredients list.
    let mut ingredients_ul = UnorderedList::builder(); // Store the builder itself
    ingredients_ul.class("ms-6 list-disc");
    for (i, ing) in r.ingredients.iter().enumerate() {
        let mut li = ListItem::builder();
        li.push(
            Span::builder()
                .data("component-kind", "ingredient")
                .data("component-ref-group", i.to_string()) // Use owned string
                .data("component-ref-target", "ingredient")
                .text(ing.name.clone()) // Clone the display name
                .build(),
        );
        if ing.modifiers().contains(Modifiers::OPT) {
            let opt_text = format!(" ({})", "OptMarker");
            li.text(opt_text); // Use owned string
        }
        if let Some(qty) = &ing.quantity {
            let qty_text = format!(": {}", qty);

            let ing_span = Span::builder()
                .data("source-units", qty.to_string())
                .text(qty_text) // Clone the display name
                .build();

            li.push(ing_span); // Use owned string
        }
        ingredients_ul.push(li.build());
    }
    content_div.push(
        Heading2::builder()
            .class("my-2 font-heading text-3xl")
            .text("Ingredients")
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
                .data("component-kind", "cookware")
                .data("component-ref-group", i.to_string()) // Use owned string
                .data("component-ref-target", "cookware")
                .text(cw.display_name().to_string()) // Clone the display name
                .build(),
        );
        cookware_ul.push(li.build());
    }
    content_div.push(
        Heading2::builder()
            .class("my-2 font-heading text-3xl")
            .push("Cookware")
            .build(),
    );
    content_div.push(cookware_ul.build());

    // Sections / Method.
    content_div.push(
        Heading2::builder()
            .class("mb-2 mt-6 font-heading text-3xl")
            .text("Steps")
            .build(),
    );
    for (sect_index, sect) in r.sections.iter().enumerate() {
        let section_id = format!("section-{}", sect_index);

        let section_index_str = sect_index.to_string();
        let section_name = sect
            .name
            .as_ref()
            .map(|s| s.clone())
            .unwrap_or_else(|| "Section".into());

        let mut sect_div = Div::builder(); // Store the builder itself
        sect_div
            .class("bg-transparent transition-colors")
            .data("section-index", section_index_str) // Use owned string
            .id(section_id); // Use owned string

        sect_div.push(
            Heading3::builder()
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
                                    let ingredient = &r.ingredients[*index];
                                    li.push(
                                        Span::builder()
                                            .class("font-semibold text-green-11")
                                            .text(ingredient.display_name().to_string())
                                            .build(),
                                    );
                                }
                                Item::Cookware { index } => {
                                    li.push(
                                        Span::builder()
                                            .class("font-semibold text-green-11")
                                            .text(r.cookware[*index].display_name().to_string())
                                            .build(),
                                    );
                                }
                                Item::Timer { index } => {
                                    li.push(
                                        Span::builder()
                                            .class("font-semibold text-green-11")
                                            .text(
                                                r.timers[*index]
                                                    .quantity
                                                    .clone()
                                                    .unwrap()
                                                    .to_string(),
                                            )
                                            .build(),
                                    );
                                }
                                Item::InlineQuantity { index } => {
                                    let ingredient_quantity = &r.inline_quantities[*index];
                                    li.push(
                                        Span::builder()
                                            .class("font-semibold text-green-11")
                                            .text(ingredient_quantity.to_string())
                                            .data("metric", ingredient_quantity.to_string())
                                            .build(),
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
