use std::error::Error;

use cooklang::{self, Converter, CooklangParser, Extensions, parser};
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

    println!("{}", current_system);
    Ok(())
}
