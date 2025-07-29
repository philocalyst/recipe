use std::error::Error;

use cooklang::{
    self, Converter, CooklangParser, Extensions, Item, Modifiers, Recipe, Step, Value,
    model::Content, scale::Scaled,
};

fn main() -> Result<(), Box<dyn Error>> {
    let parser = CooklangParser::new(Extensions::all(), Converter::bundled());

    let input = include_str!("./test.cook");

    // Base meat version
    let recipe = parser
        .parse_with_options(
            input,
            cooklang::ParseOptions::default(),
            Some("meat".to_string()),
            vec![],
        )
        .into_result()?;

    // Vegan version with all the extras
    let recipe = parser
        .parse_with_options(
            input,
            cooklang::ParseOptions::default(),
            Some("vegan".to_string()),
            vec![
                "spicy".to_string(),
                "herbs".to_string(),
                "garnish".to_string(),
                "bread".to_string(),
            ],
        )
        .into_result()?;

    // Default (no variation selected, just modifiers)
    let recipe = parser
        .parse_with_options(input, cooklang::ParseOptions::default(), None, vec![])
        .into_result()?;

    println!("{:#?}", recipe.0.sections[0]);

    Ok(())
}
