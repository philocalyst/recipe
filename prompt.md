Hey! You're tasked with archiving a recipe into a new format designed for long-term preservation: cooklang markup. This format, described below, will be the lingua de franca of cooking, meaning the quality of your conversion will determine the reading experience for millions of future readers, across generations. A promotion is on the line, and if you're able to nail this, not only will you perserve this family recipe for milennia, but you'll also secure a much-needed paycheck to provide food for your starving family!

## About Cooklang

Cooklang is the markup language at the center of an open-source ecosystem for cooking and recipe management. In Cooklang, each text file is a recipe written as plain-english instructions with markup syntax to add machine-parsible information about required ingredients, cookware, time, and metadata.

## The .cook recipe specification

Below is the specification for defining a recipe in Cooklang.

### Ingredients

To define an ingredient, use the `@` symbol. If the ingredient's name contains multiple words, indicate the end of the name with `{}`.

```cooklang
Then add @salt and @ground black pepper{} to taste.
```

To indicate the quantity of an item, place the quantity inside `{}` after the name.

```cooklang
Poke holes in @potato{2}.
```

To use a unit of an item, such as weight or volume, add a `%` between the quantity and unit.

```cooklang
Place @bacon strips{1%kg} on a baking sheet and glaze with @syrup{1/2%tbsp}.
```

Now you can try Cooklang and experiment with a few things in the Cooklang Playground!

### Steps

Each paragraph in your recipe file is a cooking step. Separate steps with an empty line.

```
A step,
the same step.

A different step.
```

Fallback

### Comments

You can add comments up to the end of the line to Cooklang text with `--`.

```cooklang
-- Don't burn the roux!

Mash @potato{2%kg} until smooth -- alternatively, boil 'em first, then mash 'em, then stick 'em in a stew.
```

Or block comments with `[- comment text -]`.

```cooklang
Slowly add @milk{4%cup} [- TODO change units to litres -], keep mixing
```

### Metadata

Recipes are more than just steps and ingredients—they also include context, such as preparation times, authorship, and dietary relevance. You can add metadata to your recipe using YAML front matter, add `---` at the beginning of a file and `---` at the end of the front matter block.

Beware: Each tag should only have lower case letters and numbers separated by a single hyphen ('-')

```yaml
---
title: Spaghetti Carbonara
tags:
  - pasta
  - quick
  - comfort food
---
```

### Cookware

You can define any necessary cookware with `#`. Like ingredients, you don't need to use braces if it's a single word.

```cooklang
Place the potatoes into a #pot.
Mash the potatoes with a #potato masher{}.
```

### Timer

You can define a timer using `~`.

```cooklang
Lay the potatoes on a #baking sheet{} and place into the #oven{}. Bake for ~{25%minutes}.
```

Timers can have a name too:

```cooklang
Boil @eggs{2} for ~eggs{3%minutes}.
```

Applications can use this name in notifications.

## Shopping Lists

To support the creation of shopping lists by apps and the command line tool, Cooklang includes a specification for a configuration file to define how ingredients should be grouped on the final shopping list. You can use `[]` to define a category name. These names are arbitrary, so you can customize them to meet your needs. For example, each category could be an aisle or section of the store, such as `[produce]` and `[deli]`.

```toml
[produce]
potatoes

[dairy]
milk
butter
```

Or, you might be going to multiple stores, in which case you might use `[Tesco]` and `[Costco]`.

```toml
[Costco]
potatoes
milk
butter

[Tesco]
bread
salt
```

You can also define synonyms with `|`.

```toml
[produce]
potatoes

[dairy]
milk
butter

[deli]
chicken

[canned goods]
tuna|chicken of the sea
```

## Conventions

There're things which aren't part of the language specification but rather common conventions used in tools build on top of the language.

### Adding Pictures

You can add images to your recipe by including a supported image file (`.png`,`.jpg`) matching the name of the recipe recipe in the same directory.

```shell
Baked Potato.cook
Baked Potato.jpg
```

You can also add images for specific steps by including a step number before the file extension.

```shell
Chicken French.cook
Chicken French.0.jpg
Chicken French.3.jpg
```

### Canonical metadata

To use your recipes across different apps, follow the conventions on how to name metadata in common cases:

| Key | Purpose | Example value |
|-----|---------|---------------|
| `source`, `source.name` | Where the recipe came from. Usually a URL, can also be text (eg. a book title). | `https://example.org/recipe`, `The Palomar Cookbook <urn:isbn:9781784720995>`, `mums` |
| `author`, `source.author` | The author of the recipe. | `John Doe` |
| `source.url` | The URL of the recipe if nested format is used. | `https://example.org/recipe` |
| `servings`, `serves`, `yield` | Indicates how many people the recipe is for. Used for scaling quantities. Leading number is used for scaling, anything else is ignored but shown as units. | `2`,`15 cups worth` |
| `course`, `category` | Meal category or course. | `dinner` |
| `locale` | The locale of the recipe. Used for spelling/grammar during edits, and for pluralisation of amounts. Uses ISO 639 language code, then optionally an underscore and the ISO 3166 alpha2 "country code" for dialect variants | `es_VE`, `en_GB`, `fr` |
| `time required`, `time` or `duration` | The preparation + cook time of the recipe. Various formats can be parsed, if in doubt use HhMm format to avoid plurals and locales. | `45 minutes`, `1 hour 30 minutes`,`1h30m` |
| `prep time`, `time.prep` | Time for preparation steps only. | `2 hour 30 min` |
| `cook time`, `time.cook` | Time for actual cooking steps. | `10 minutes` |
| `difficulty` | Recipe difficulty level. | `easy` |
| `cuisine` | The cuisine of the recipe. | `French` |
| `diet` | Indicates a dietary restriction or guideline for which this recipe or menu item is suitable, e.g. diabetic, halal etc. | `gluten-free`, or array of values |
| `tags` | List of descriptive tags. | `[2022, baking, summer]` |
| `image`, `images`, `picture`, `pictures` | URL to a recipe image. | `https://example.org/recipe_image.jpg` or array of URLs |
| `title` | Title of the recipe. | `Uzbek Manti` |
| `introduction`, `description` | Additional notes about the recipe. | `This recipe is a traditional Uzbek dish that is made with a variety of vegetables and meat.` |

## Advanced

### Notes

To include relevant background, insights, or personal anecdotes that aren't part of the cooking steps, use notes. Start a new line with `>` and add your story.

```cooklang
> Don't burn the roux!

Mash @potato{2%kg} until smooth -- alternatively, boil 'em first, then mash 'em, then stick 'em in a stew.
```

### Sections

Some recipes are more complex than others and may include components that need to be prepared separately. In such cases, you can use the section syntax, e.g., `==Dough==`. The section name and the `=` symbols after it are optional, and the number of `=` symbols does not matter. By default, all instructions are inside an anonymously named section, so any added sections are effectively 2-indexed. (As below, dough would actually be section two, filling section three)

```cooklang
= Dough

Mix @flour{200%g} and @water{100%ml} together until smooth.

== Filling ==

Combine @cheese{100%g} and @spinach{50%g}, then season to taste.
```

### Short-hand preparations

Many recipes involve repetitive ingredient preparations, such as peeling or chopping. To simplify this, you can define these common preparations directly within the ingredient reference using shorthand syntax:

```cooklang
Mix @onion{1}(peeled and finely chopped) and @garlic{2%cloves}(peeled and minced) into paste.
```

### Referencing other recipes

You can reference other recipes using the existing `@` ingredient syntax, inferring relative file paths from the ingredient name:

```cooklang
Pour over with @./sauces/Hollandaise{150%g}.
```

These preparations should be clearly displayed in the ingredient list, allowing you to get everything ready before you start cooking.

# Cooklang syntax extensions

## Modifiers
With the ingredient modifiers you can alter the behaviour of ingredients. There
are 5 modifiers:
- `@` **Recipe**. References another recipe by it's name.
  ```cooklang
  Add @@tomato sauce{200%ml}.
  ```
- `&` **Reference**. References another ingredient with the same name. If a
  quantity is given, the amount can be added. The ingredient must be defined
  before. If there are multiple definitions, use the last one.
  ```cooklang
  Add @flour{200%g} [...], then add more @&flour{300%g}.
  ```
- `-` **Hidden**. Hidden in the list, only appears inline.
  ```cooklang
  Add some @-salt.
  ```
- `?` **Optional**. Mark the ingredient as optional.
  ```cooklang
  Now you can add @?thyme.
  ```
- `+` **New**. Forces to create a new ingredient. This works with the
  [modes](#modes) extension.

This also works (except recipe) for cookware.

## Intermediate preparations
You can refer to intermediate preparations as ingredients. For example:
```cooklang
Add @flour{200%g} and @water. Mix until combined.

Let the @&(~1)dough{} rest for ~{1%hour}.
```
Here, `dough` is refering to whatever was prepared one step back.
These ingredients will not appear in the list.

There are more syntax variations:
```cooklang
@&(~1)thing{}  -- 1 step back
@&(2)thing{}   -- step number 2
@&(=2)thing{}  -- section number 2
@&(=~2)thing{} -- 2 sections back
```

Only past steps from the current section can be referenced. It can only be
combined with the optional (`?`) modifier. Text steps can't be referenced. In
relative references, text steps are ignored. Enabling this extension
automatically enables the [modifiers](#modifiers) extension. As these are not stored, if you make another
reference, you'll need to use the full form, not merely @&thing{}, or it will be a parser error.

## Component alias
Add an alias to an ingredient to display a different name.

```cooklang
@white wine|wine{}
@@tomato sauce|sauce{}     -- works with modifiers too
```

This can be useful with references. Here, the references will be displayed as
`flour` even though the ingredient it's refering is `tipo zero flour`.

```cooklang
Add the @tipo zero flour{}
Add more @&tipo zero flour|flour{}
```

This also works for cookware.

## Advanced units
Maybe confusing name. Tweaks a little bit the parsing and behaviour of units
inside quantities.

- When the value is a number or a range and the values does not start with a
number, the unit separator (`%`) can be replaced with a space.
  ```cooklang
  @water{1 L} is the same as @water{1%L}
  ```

  If disabled, `@water{1 L}` would parse as `1 L` being a text value.
- Enables extra checks:
  - Checks that units between references are compatible, so they can be added.
  - Checks that timers have a time unit.

## Modes
Add new special metadata keys that control some of the other extensions. The
special keys are between square brackets.

```cooklang
>> [special key]: value
```

- `[mode]` | `[define]`
  - `all` | `default`. This is the default mode, same as the original cooklang.
  - `ingredients` | `components`. In this mode only components can be defined,
  all regular text is omitted. Useful for writing an ingredient list manually
  at the beginning of the recipe if you want to do so.
  - `steps`. All the ingredients are references. To force a new ingredient, use
  the new (`+`) modifier.
  - `text`. All steps are [text blocks](#text-blocks)

- `duplicate`
  - `new` | `default`. When a ingredient with the same name is found, create a
  new one. This is the original cooklang behaviour.
  - `reference` | `ref`. Ingredients have implicit references when needed. So
  ingredients with the same name will be references. To force a new ingredient,
  use the new (`+`) modifier.
    ```cooklang
    >> [duplicate]: ref
    @water{1} @water{2}
    -- is the same as
    >> [duplicate]: default
    @water{1} @&water{2}
    ```

## Temperature
Find temperatures in the text, without any markers. In the future this may be
extended to any unit.

For example, the temperature here will be parsed[^2] not as text, but as an inline
quantity.
```cooklang
Preheat the #oven to 180 ºC.
```

## Range values
Recipes are not always exact. This is a little improvement that should help
comunicating that in some cases.

```cooklang
@eggs{2-4}
@tomato sauce{200-300%ml}            -- works with units
@water{1.5-2%l}                      -- with decimal numbers too
@flour{100%g} ... @&flour{200-400%g} -- the total will be 300-500 g
```

## Timer requires time
Just an extra rule that makes timers like `~name` invalid.

[^1]: This is work in progress in `cooklang` but supported here.

[^2]: Currently this is done in the analysis pass. So in the AST there is no
concept of inline quantities.

### Name with URL

Example: `Mom's Cookbook <https://moms-cookbook.url>` -> name: `Mom's Cookbook` url: `https://moms-cookbook.url/`

The interpretations of the key value will be:

- `name <valid url>` -> as `name` & `url`
- `name <invalid url>` -> as `name`
- `name` -> as `name`
- `invalid url` -> as `name`
- `<invalid url>` -> as `name`
- `valid url` -> as `url`
- `<valid url>` -> as `url`

Some examples:

## The original:
Banana Nut Bread by Michael Chu
Banana nut breads come in all varieties. Popular recipes result in textures ranging from moist to cakey. Personally, I like banana nut bread that is denser than a classic yellow cake, but not quite as dry as wheat bread. The banana nut bread should be tender and flavorful, but not have the consumer feel like he needs to drink a glass of water with it. Some of the recipes that I've tried were so moist that the "bread" sticks to the roof of the mouth, while other recipes were much too dry - both require drinking a glass of water to get the bread down. (Of course, drinking a glass of milk while eating a slice of good banana bread is an awesome combination, but it shouldn't be considered a necessity for enjoying banana nut bread.) This recipe results in what I feel is the perfect combination of flavor and texture in banana nut bread.

Banana nut bread should always start with fully ripe bananas. Unfortunately, ripe bananas are not usually sold in the supermarket. While a banana ripens, the starch of the banana slowly converts to sugars. Allow green or yellow bananas to ripen at room temperature until the skin is liberally covered with brown spots. Once the banana has reached this stage, it is fully ripe. Bananas can be frozen once they have reached the desired ripeness. Their peels will turn completely brown, but don't worry about the banana within. When ready to use, simply thaw the bananas by letting them sit (unpeeled) on the counter until they warm up. Once thawed, peel the bananas.

Start by preparing a loaf pan by buttering the bottom and sides. Lightly flour the pan and tap out the excess flour. The loaf pan should be around 5 in. by 9 in. (13 cm by 23 cm) in size - a little larger or smaller isn't a problem.

The wet ingredients are: two ripe bananas, 6 Tbs. melted butter, 1 tsp. vanilla extract, and two large eggs. For the dry ingredients: 1-1/3 cup flour, 1/2 tsp. baking soda, 1/4 tsp. baking powder, 2/3 cup sugar, 1/2 tsp. salt, and 1/2 cup chopped walnuts. Combine and whisk all the dry ingredients except for the walnuts. The use of both baking soda and powder are necessary to provide enough leavening for the proper consistency of the bread. The baking soda is just enough to utilize the slight acidity of the bananas to create the desired carbon dioxide bubbles. Baking powder (which is a mix of baking soda, a base, and cream of tartar, an acid) provides even more leavening power.


Mash the bananas, melted butter, and vanilla extract together. Lightly beat the eggs together.


Mash the banana mixture with the eggs until smooth and well blended.


Pour the banana mixture onto the dry ingredients. Add the walnuts.


Fold the ingredients together until no more white flour is uncovered while folding.


Pour the batter into the prepared loaf pan and bake for 55 minutes at 350°F.


After 55 minutes, the loaf of banana bread should be done. A wooden toothpick inserted into the center should come out clean. Set the pan on a wire rack to cool for ten minutes.


Remove the loaf from the pan and let cool on the wire rack. Serve warm or fully cooled. The loaf can be wrapped in plastic and stored at room temperature for about four or five days.
 
## The converted version: 
---
title: Banana Nut Bread
description: >
  Banana nut breads come in all varieties. Popular recipes result in textures
  ranging from moist to cakey. Personally, we like banana nut bread that is
  denser than a classic yellow cake, but not quite as dry as wheat bread. The
  banana nut bread should be tender and flavorful, but not have the consumer
  feel like he needs to drink a glass of water with it. Some of the recipes
  that we've tried were so moist that the bread sticks to the roof of the mouth,
  while other recipes were much too dry - both require drinking a glass of
  water to get the bread down. (Of course, drinking a glass of milk while
  eating a slice of good banana bread is an awesome combination, but it
  shouldn't be considered a necessity for enjoying banana nut bread.) This
  recipe results in what we feel is the perfect combination of flavor and
  texture in banana nut bread.
servings: 10
locale: en_US
diet:
  - vegetarian
  - nut-containing
  - dairy-containing
  - egg-containing
tags:
  - baking
  - snack
  - sweet
cuisine: American
time: 75 minutes
time.prep: 15 minutes
time.cook: 55 minutes
course: bread
difficulty: moderate
---

> Banana nut bread should always start with fully ripe bananas. Unfortunately, ripe bananas are not usually sold in the supermarket. While a banana ripens, the starch of the banana slowly converts to sugars. Allow green or yellow bananas to ripen at room temperature until the skin is liberally covered with brown spots. Once the banana has reached this stage, it is fully ripe. Bananas can be frozen once they have reached the desired ripeness. Their peels will turn completely brown, but don't worry about the banana within. When ready to use, simply thaw the bananas by letting them sit (unpeeled) on the counter until they warm up. Once thawed, peel the bananas.

== Preperation ==

Start by preparing a #loaf pan(5inx9in){1} by buttering the bottom and sides. Lightly flour the #&loaf pan(5inx9in){} and tap out the excess flour.

Combine and whisk all the dry ingredients except for the walnuts: @all-purpose flour{1-1/3%cups}, @sugar{2/3%cup}, @baking soda{1/2%tsp}, @baking powder{1/4%tsp}, and @salt{1/2%tsp}.

> The use of both baking soda and powder are necessary to provide enough leavening for the proper consistency of the bread. The baking soda is just enough to utilize the slight acidity of the bananas to create the desired carbon dioxide bubbles. Baking powder (which is a mix of baking soda, a base, and cream of tartar, an acid) provides even more leavening power.

== Create the Banana Mixture == 
Mash @bananas(ripe,large){2}, @butter(melted){6%Tbs}, and @vanilla extract{1%tsp} together.

Lightly beat the @eggs(large){2} together.

== Create the Batter == 

Mash the @&(=~1)banana mixture{} with the @&eggs(large){} until smooth and well blended.

Pour the @&(=~1)banana mixture{} onto the dry ingredients. Add @walnuts(chopped){1/2%cup}.

Fold together until no more white flour is uncovered while folding.

== Bake ==

Pour the @&(=~1)batter into the prepared #&loaf pan(5inx9in){} and bake (in the #oven{}) for ~{55%minutes} at 350ºF.

== Serve ==

After ~{55%minutes}, the @&(=~1)loaf{} of banana bread should be about done. Set the #&loaf pan(5inx9in)|pan{} on the #wire rack{} to cool for ~{10%minutes}.

> You can know if the bread is cooked through when a wooden toothpick inserted into the center comes out clean.

Remove the loaf from the #&loaf pan(5inx9in)|pan{} and let cool on the #&wire rack{}. Serve warm or fully cooled.

> The loaf can be wrapped in plastic and stored at room temperature for about four or five days.

## Original

 Chocolate Cake by Michael Chu
Making a beautiful cake is an art, and, like any art, it takes a great deal of innate talent and lots of practice to create. However, regardless of artistic inclination, everyone should be able to learn how to produce a great tasting cake by following the right recipe. This recipe for chocolate cake has a strong, but not overwhelming, chocolate taste that will put a smile on any chocolate lover.

The chocolate used in this cake is fairly important. Dutch processed chocolate has a different alkalinity than unprocessed chocolate and cacao powder, so these ingredients cannot be easily interchanged in this recipe. Use a high quality eating chocolate (such as Dagoba) and pure cacao powder.

Assemble the ingredients for the batter: 8 oz. (225 g) unsalted butter (softened), 1-1/4 cups (295 g) drinking water, 3/4 cup (105 g) all-purpose flour, 3/4 cup (85 g) cake flour, 1-1/2 cup (300 g) dark brown sugar, 3 large eggs, 1/4 cup (20 g) pure cacao powder, 2 oz. (55 g) 70% cacao dark chocolate, 1-1/4 tsp. baking soda, 1/4 tsp. table salt, 1/2 cup (115 g) sour cream, and 1 tsp. (5 mL) pure vanilla extract.


We'll also need two 9-in. round cake pans (or 3 8-in. round cake pans for a taller three-layer cake) and two sheets of parchment paper. Place a cake pan over each sheet of parchment paper. Using a pencil, trace a circle around the base of the cake pan.


Fold the parchment paper in half so that the circle forms a half circle. Then fold in half again to form a 90° arc. Cut along this curve to form circles of parchment paper.


The paper should fit perfectly into the bottoms of each cake pan. Using this folding and cutting technique is a bit simpler than attempting to neatly cut and entire circle.


Grease the bottom and sides of each cake pan with the wrappers from the unsalted butter. (You can do this step now or after you cream the butter - whatever is convenient.)


Line the bottom of each pan with the circles of parchment paper.


Start by sifting the two flours, baking soda, and salt together. I like to sift them onto a sheet of wax or parchment paper because the paper can then be picked up and the contents poured out in an easy to control manner.


Break up the chocolate into 1/2-inch (1 to 1.5 cm) squares and combine with the cacao powder.


Bring the water to a boil and measure out 1-1/4 cup. Pour over the chocolate and whisk gently until the chocolate has completely dissolved. Using water heightens the chocolate flavor of the mixture. (Try tasting chocolate melted in warm milk and compare it to the taste of chocolate melted in water. The milk based hot chocolate will feel thicker and richer, but the water based hot chocolate will have a surprisingly strong chcolate flavor.)


Once the flour has been sifted and the chocolate melted into boiling water, cream the unsalted butter with a standing mixer equipped with a flat beater. Add the dark brown sugar and mix until butter and sugar are evenly mixed. Scrape down the bowl with a spatula.


One at a time, add the eggs and beat on medium-high until fully incorporated.


Mix in sour cream and vanilla extract. Scrape the bowl down.

On low speed, mix in a third of the flour mixture followed by half of the chocolate liquid. Repeat with another third of the flour and the rest of the chocolate. Finally, mix in the last third of the flour. Stop the mixer once the batter has just combined.


Pour the batter into each cake pan as evenly as possible. (For my readers who love to be as precise as possible, the fastest way to split the batter evenly is to tare the weight of a cake pan on a digital scale and alternate pouring between two pans while massing them in turn. In about thirty seconds, you can evenly split the batter with as little as 1% error.)


Bake both cake pans in a 350°F (175°C) oven on a rack set to the center position for 25 minutes or until a toothpick or wood skewer thrust into the center of the cake and withdrawn is clean or only has dry crumbs attached. Remove both pans from the oven and allow them to site on a wire rack for five minutes.


Run a knife along the circumference of each round to release the cake from the pan. Invert the pan over a wire rack. The cake should gently release and rest on the rack.


Immediately remove the parchment paper from the bottom of the cake. Waiting until the cake begins to cool may result in some of the cake surface sticking to the paper and lifting off as you remove the paper.


Allow the cake rounds to cool completely. Meanwhile, prepare a batch of heavy frosting such as a buttercream to frost the cake.

Once the cake layers have fully cooled, apply a little frosting to the middle of a cake cardboard, plate, or whatever surface you plan on constructing your cake. In these pictures, I built my cake directly onto a cake keeper.


Place one of the cake layers onto the dollop of frosting. The frosting acts as mortar - keeping the cake from moving while we work with it. I like to place the layer topside down. This is because the bottom of the cake layer will be facing up and the next layer of cake will be placed bottom side down. Since I use two identical cake pans, the dimensions of the bottoms of the cakes will always match. If the cake layers are not relatively flat, you can use a long serrated knife (like a bread knife) to cut any excess cake that might form a dome. Perform any cutting on the topside (the side resting on the wire rack).


Place a sizable quantity of frosting onto the top of the cake layer (exact quantity will vary, but make sure you use less than half of your total frosting).


Using an icing spatula or offset spatula, spread the frosting so it forms a flat layer. I find this easiest by holding the spatula so the edge is held at a 45° angle to the surface of the cake and the frosting is pushed out from the middle of the cake. I keep pushing the frosting out and rotating the platform that the cake is sitting on.


Lay the second layer of cake onto the leveled frosting. Be sure to match the face of the cakes (bottom down if the first cake layer was placed top down) so the diameters will match.


Apply frosting evenly to the sides of the cake. Use the rest of the frosting to cover the top of the cake. At this point you can practice the art of cake decoration - of which I am not well practiced.


Despite the amateur appearance of the cake, the flavor of this cake can only be described as really chocolatey without any bitterness. The texture is halfway between crumbly and moist - dense but not heavy.

## Converted Version:
---
title: Chocolate Cake
author: Michael Chu
description: >
  Making a beautiful cake is an art, and, like any art, it takes a great deal of
  innate talent and lots of practice to create. However, regardless of artistic
  inclination, everyone should be able to learn how to produce a great tasting
  cake by following the right recipe. This recipe for chocolate cake has a
  strong, but not overwhelming, chocolate taste that will put a smile on any
  chocolate lover.
servings: 12
locale: en_US
diet:
  - vegetarian
  - dairy-containing
  - egg-containing
tags:
  - chocolate
  - baking
  - cake
  - dessert
cuisine: American
time: 90 minutes
time.prep: 60 minutes
time.cook: 25 minutes
course: dessert
difficulty: moderate
---

> The chocolate used in this cake is fairly important. Dutch processed chocolate has a different alkalinity than unprocessed chocolate and cacao powder, so these ingredients cannot be easily interchanged in this recipe. Use a high quality eating chocolate (such as Dagoba) and pure cacao powder.

== Preparation ==

Place a #cake pan(9in,round){1} (or #?cake pans(8in,round){3} for a taller three-layer cake) over each sheet of #parchment paper{2}.

Using a #pencil{}, trace a circle around the base of the #&cake pan(9in,round){}.

Fold the #&parchment paper|sheet{} in half so that the circle forms a half circle. Then fold in half again to form a 90° arc. Cut along this curve to form circles in the #&parchment paper|sheet{}.

> The paper should fit perfectly into the bottoms of each cake pan(9in,round). Using this folding and cutting technique is a bit simpler than attempting to neatly cut and entire circle.

Grease the bottom and sides of each #&cake pan(9in,round){} with the wrappers from the @unsalted butter(softened){8%oz}. Line the bottom of each #&cake pan(9in,round)|pan{} with the circles of the #&parchment paper|sheet{}.

== Dry Ingredients ==

Start by sifting @all-purpose flour{3/4%cup}, @cake flour{3/4%cup}, @baking soda{1-1/4%tsp}, and @table salt{1/4%tsp} together onto a sheet of #wax paper{}.

== Chocolate Sauce ==

In a #bowl(small){1}, break up @dark chocolate(70% cacao){2%oz} into 1/2-inch squares and combine with @pure cacao powder{1/4%cup}.

Bring @water(drinking){1-1/4%cups} to a boil. Pour over the @&(~1)chocolate mixture{} and whisk gently until the chocolate has completely dissolved.

> Using water heightens the chocolate flavor of the mixture. Try tasting chocolate melted in warm milk and compare it to the taste of chocolate melted in water. The milk based hot chocolate will feel thicker and richer, but the water based hot chocolate will have a surprisingly strong chocolate flavor.

== Create the Batter ==

Cream the @&unsalted butter(softened){} with a #standing mixer(equipped with a flat beater){} in a #bowl(large)|bowl{1}. Add @dark brown sugar{1-1/2%cups} and mix until butter and sugar are evenly mixed.

Scrape down the #&bowl(large)|bowl{} with a #spatula{}.

One at a time, add @eggs(large){3} and beat on medium-high until fully incorporated.

Mix in @sour cream{1/2%cup} and @pure vanilla extract{1%tsp}. Scrape the #&bowl(large){} down.

On low speed, mix in a third of the @&(=~2)flour mixture{} followed by half of the @&(=~1)chocolate liquid{}.

Repeat with another third of the @&(=~2)flour mixture{} and the rest of the @&(=~1)chocolate liquid{}. Finally, mix in the last third of the @&(=~2)flour mixture{}. Stop the mixer once the batter combines.

== Bake ==

Pour the @&(=~1)batter{} into each #&cake pan(9in,round){} as evenly as possible.

> For readers who love to be as precise as possible, the fastest way to split the batter evenly is to tare the weight of a cake pan(9in,round) on a digital scale and alternate pouring between two pans while massing them in turn. In about thirty seconds, you can evenly split the batter with as little as 1% error.

Bake both #&cake pan(9in,round){}s in a 350°F oven on a rack set to the center position for ~{25%minutes} or until a #?toothpick{} thrust into the center of the cake and withdrawn is clean or only has dry crumbs attached.

Remove both #&cake pan(9in,round){}s from the #oven{} and allow them to sit on a #wire rack{} for ~{5%minutes}.

== Remove from Pans ==

Run a #knife{} along the circumference of each round to release the cake from the #&cake pan(9in,round)|pan{}. Invert the #&cake pan(9in,round)|pan{} over a #&wire rack{}. The cake should gently release and rest on the rack.

Immediately remove the #&parchment paper{} from the bottom of the cake.

> Waiting until the cake begins to cool may result in some of the cake surface sticking to the paper and lifting off as you remove the paper.

Allow the @&(=~1)cake rounds{} to cool completely. Meanwhile, prepare a batch of @frosting(heavy){} such as a buttercream to frost the cake.

== Assembly ==

Once the @&(=~2)cake layers{} have fully cooled, apply a little @&frosting(heavy){} to the middle of a #?cake cardboard{} or #?plate{1}.

Place one of the @&(=~2)cake layers{} onto the dollop of @&frosting(heavy){}. Place the layer topside down.

> The frosting acts as mortar - keeping the cake from moving while we work with it. Since we use two identical cake pans, the dimensions of the bottoms of the cakes will always match. If the cake layers are not relatively flat, you can use a long serrated knife to cut any excess cake that might form a dome.

Place a sizable quantity of @&frosting(heavy){} onto the top of the @&(=~2)cake layer|layer{}.

Using an #?icing spatula{}, spread the @&frosting(heavy){} so it forms a flat layer.

> We find this easiest by holding the spatula so the edge is held at a 45° angle to the surface of the cake and the frosting is pushed out from the middle of the cake. We keep pushing the frosting out and rotating the platform that the cake is sitting on.

Lay the second @&(=~2)cake layer|layer{} onto the leveled @&frosting(heavy){}. Be sure to match the face of the cakes so the diameters will match.

Apply @&frosting(heavy){} evenly to the sides of the @&(~1)cake. Use the rest of the @&frosting(heavy){} to cover the top of the @&(~1)cake.

> Despite the amateur appearance of the cake, the flavor of this cake can only be described as really chocolatey without any bitterness. The texture is halfway between crumbly and moist - dense but not heavy.

## Original

Garlic bread can be a great addition to you dinner. These days, it seems easiest to just go to the local supermarket and pick up a loaf of garlic bread. However, the last time I checked out the ingredient list of my local supermarket chain's garlic bread, one of the primary ingredients was partially hydrogenated vegetable oil. Why eat the supermarket stuff when it's so easy to convert a plain loaf of bread into garlicky goodness?

This recipe works on a 16 oz. loaf of bread (the most common size sold in stores in my area). I find that a thin baguette (or ficelle) is too narrow for the garlic bread to be really satisfying. Using a typical artisan baguette or a batard works best.

Related Articles
Orzo Risotto with Buttery Shrimp
Asparagus with Almonds
Shepherd's Pie (Cottage Pie)
Cheesecake, Plain New York Style
Lemon Bars
This recipe goes pretty quick, so before assembling the ingredients you may want to start preheating the oven to 350°F (175°C).

Gather up a loaf of French bread, 1/2 cup (55 g) grated mozzarella cheese, 1/4 cup (25 g) grated parmesan cheese, 2 cloves garlic, and 2 Tbs. butter.


Either press the garlic cloves through a garlic press and then finely mince unto a pulp or grate the garlic cloves with a Microplane zester. For a milder garlic flavor, microwave the garlic for thirty seconds or more. Mash the garlic into the butter. If the butter is too cold to mash easily, microwave the butter for ten seconds and try again. Mix the garlic and butter together well.


Slice the bread in half along its length.


Spread the butter and garlic mixture onto both cut sides of the bread. There should be just enough of the garlic butter to form a thin layer.


Place the two loaves on a sheet pan. Sprinkle the parmesan cheese evenly over both halves of the loaf. Sprinkle the mozzarella over the parmesan.


Slide the pan into the oven and bake for 20 minutes. The cheese should have melted and just begun to brown lightly.


Remove from the oven and let rest for a few minutes before cutting. Cut each half into about a dozen pieces.


For a variation, dried herbs can be added to the bread before baking. Or try it without cheese for a lighter taste.

## Converted
---
title: Garlic Bread
description: >
  Garlic bread can be a great addition to your dinner. These days, it seems
  easiest to just go to the local supermarket and pick up a loaf of garlic
  bread. However, the last time we checked out the ingredient list of our local
  supermarket chain's garlic bread, one of the primary ingredients was partially
  hydrogenated vegetable oil. Why eat the supermarket stuff when it's so easy to
  convert a plain loaf of bread into garlicky goodness?
servings: 8
locale: en_US
diet:
  - vegetarian
  - dairy-containing
tags:
  - garlic
  - bread
  - side-dish
  - baking
  - quick
cuisine: Italian-American
time: 30 minutes
time.prep: 10 minutes
time.cook: 20 minutes
course: side dish
difficulty: easy
---

> This recipe works on a 16 oz. loaf of bread (the most common size sold in stores). A thin baguette (or ficelle) is too narrow for the garlic bread to be really satisfying. Using a typical artisan baguette or a batard works best.

Preheat the #oven{} to 350°F.

> The preperation is relatively quick, so preheating the oven saves time.

== Garlic Butter ==

Either press @garlic{2%cloves} through a #garlic press{} and then finely mince unto a pulp or grate the @&garlic{} with a #?Microplane zester{1}. 

> For a milder garlic flavor, microwave the garlic for thirty seconds or more.

Mash the @&garlic{} into the @butter{2%Tbs}. Mix them well.

> If the butter is too cold to mash easily, microwave the butter for 10 seconds and try again. 

== Prepare Bread ==

Slice @French bread(16 oz loaf)|bread{1} in half along its length with a #knife{1}.

Spread the @&(=~1)garlic butter mixture{} onto both cut sides of the @&French bread(16 oz loaf)|bread{}. There should be just enough of the @&(=~1)garlic butter mixture|mixture{} to form a thin layer.

== Assemble and Bake ==

Place the two @&(=~1)bread halves{} on a #sheet pan{1}. Sprinkle @parmesan cheese(grated){1/4%cup} evenly over both halves of the @&(=~1)bread halves|loaf{}. Sprinkle @mozzarella cheese(grated){1/2%cup} over the @&parmesan cheese(grated){}.

Once the @&(~1)cheese has melted and begun to brown lightly, slide the #&sheet pan|pan{} into the #&oven{} and bake for ~{20%minutes}. 

== Serve ==

Remove from the #&oven{} and let rest for a ~{5 minutes} before cutting. Cut each half into about a dozen pieces.

> For a variation, dried herbs can be added to the bread before baking. Or try it without cheese for a lighter taste.

## Original
I love spicy buffalo chicken wings. I also love chili. And yes, I love to cook too. So when the company I work for (Boeing) presented a chili cook-off contest a while back, I took the challenge to heart (and the drawing board!). There are a few versions of buffalo chicken chili circulating the web, but I came up with a variation that I think adds much more texture, intense flavor, and plenty of heat while minimizing the labor and time. And best of all, this chili recipe is quite healthy. (Oh, and I won the cook-off with this chili.)

I did follow the traditional "Texas" style of chili making and did not include beans. This recipe is very thick, savory, and meaty. One of my variations is to use two types of chicken meat for added flavor and texture. I use the traditional ground chicken, and I also include whole shredded chicken from a store bought rotisserie chicken. If you prefer beans in your chili, please feel free to add them (I think pinto beans would be very good). I included plenty of aromatics (vegetables) cooked until soft and brown, and a good dose of Louisiana hot pepper sauce for heat. The addition of beer adds some flavor but mostly assist in deglazing the pan which is very important in this recipe since I recommend cooking this chili in a stainless steel pan (do not use a non-stick pan) in order to generate lots of those yummy brown bits (fond) that stick to the bottom of the pan (which is always a good thing!).

Starting with a store bought rotisserie chicken, shred the chicken from the bones (discarding the skin), and temporarily store the meat in a bowl. If you feel the need to roast your own chicken go for it, but in this case, I think a store bought roasted chicken saves lots of time and energy.

Some people (like my daughter) prefer to opt for larger cuts of meat and cut the chicken into cubes rather than shred. This option is up to you. I prefer the shredded meat because it adds a thicker texture to the chili, and also adds more surface area to blend with sauce.


For the aromatics (vegetables), start with 2 carrots, 3 celery stalks, 1 red bell pepper, and 1 medium onion.

The addition of diced jalapeño pepper(s) is optional for extra heat.

Cut the vegetables into a fine dice.

(Discard the seeds of the red bell pepper.)

Once again, the fine dice adds more surface area to the chili, which adds a more thicker texture, and richer flavor when sautéed.


Next, mince 5 cloves of garlic.

For the spices, you will need 2 Tbs of Chili Powder, 3 tsp of ground cumin, and 1 tsp of ground coriander.

Add additional spices (plus salt and pepper) as your taste desires when the chili is completed.


Finally you will need one 14.5 oz. can of diced tomatoes, one 15 oz. can of tomato sauce, 1/2 cup of Louisiana cayenne hot pepper sauce (give or take), and one 12 oz bottle of beer for deglazing the pan.


Start by cooking the ground chicken meat in a med-high heatedpan with a little olive oil.

I prefer a stainless steel pan so that the meat will form little browned bits (called fond) that will stick to the bottom of the pan and provide lots of concentrated flavor later on.

Season the meat with salt and pepper.

Make sure the meat gets good and brown (Browning = Flavor).


Related Articles
Metrokane Mighty OJ (Not recommended)
Kitchen Mysteries by Herve This
Lime Marinated Grilled Chicken
Beef Stroganoff
Salsa Cruda
When the meat is cooked, reserve the chicken for later use.

Use the same pan to brown the vegetables in the next step, in order to get the browned bits released from the bottom of the pan.

Add 3 Tbs of butter to the pan and cook the vegetables over med-high heat, for at least 10 to 15 minutes, until the vegetables are soft, tender, and the onions start to brown.

Once again, add some salt (and/or pepper).

With a wooden spoon, scrape up all of the brown bits from the cooked meat which will add intense flavor to the chili.

Browning equals flavor, so do not rush this step.


Note the brown bottom on the stainless steel pan when you are finished cooking the meat and vegetables. This is pure flavor!

The beer will be used to deglaze the pan, and add this concentrated flavor to the chili.


Add the cooked chicken and vegetables back to the pan and clear a spot in the center of the pan to cook the spices for 30 seconds.

Add about 1 Tbs of olive oil to the center of the pan then add the garlic and spices. Stir around and cook for about 30 seconds.


At this point, the bottom of the pan will be very brown with food and spices sticking.

Add 12oz of a good beer to the pan and stir with a wooden spoon to deglaze the pan, and dissolve all of the brown bits stuck on the bottom.


Finally add the tomato sauce, diced tomatoes, and hot sauce.

Simmer to the desired thickness that you prefer.

Season with salt and pepper to taste.

Add additional spices if desired.


Garnish with your favorite condiments.


I prefer sour cream, chopped green onions, and served with a warm bread stick.

## Converted
---
title: ---
title: Buffalo Chicken Chili
description: >
  A spicy buffalo chicken chili that combines the flavors of buffalo chicken
  wings with traditional chili. This recipe uses two types of chicken meat for
  added flavor and texture, follows traditional Texas-style chili making
  without beans, and creates a thick, savory, and meaty chili with plenty of
  heat. The addition of beer helps deglaze the pan to capture all those flavorful
  brown bits that develop during cooking.
servings: 6
locale: en_US
diet:
  - meat-containing
  - dairy-containing
  - gluten-free
tags:
  - spicy
  - chili
  - buffalo
  - chicken
  - texas-style
cuisine: American
time: 45 minutes
time.prep: 15 minutes
time.cook: 30 minutes
course: main dish
difficulty: moderate
---

> This recipe follows the traditional "Texas" style of chili making and does not include beans. The recipe is very thick, savory, and meaty. One variation is to use two types of chicken meat for added flavor and texture - traditional ground chicken and whole shredded chicken from a store bought rotisserie chicken. If you prefer beans in your chili, feel free to add them (pinto beans would be very good).

> The addition of beer adds some flavor but mostly assists in deglazing the pan which is very important in this recipe. We recommend cooking this chili in a stainless steel pan (do not use a non-stick pan) in order to generate lots of those yummy brown bits (fond) that stick to the bottom of the pan.

== Preparation ==

Starting with a @rotisserie chicken(store-bought){1}, shred the meat from the bones (discarding the skin), and temporarily store the meat in a #bowl{1}.

> If you feel the need to roast your own chicken go for it, but in this case, a store bought roasted chicken saves lots of time and energy. Some people prefer to opt for larger cuts of meat and cut the chicken into cubes rather than shred. We prefer the shredded meat because it adds a thicker texture to the chili, and also adds more surface area to blend with sauce.

Cut @carrots{2}, @celery stalks{3}, @red bell pepper{1}, and @onion(medium){1} into a fine dice. Discard the seeds of the @&red bell pepper{}. Mix in the @?jalapeño peppers(diced){}.

> The addition of diced jalapeño pepper(s) is optional for extra heat. The fine dice adds more surface area to the chili, which adds a more thicker texture, and richer flavor when sautéed.

Mince @garlic{5%cloves}.

== Cooking the Meat ==

Cook the @chicken(ground){1%lb} in a #pan(stainless steel){1} at medium to high heat with a little @olive oil{}. Season the meat with @salt{} and @black pepper{}.

> Make sure the meat gets good and brown (Browning = Flavor). A stainless steel pan is preferred so that the meat will form little browned bits (called fond) that will stick to the bottom of the pan and provide lots of concentrated flavor later on.

When the @&(~1)meat{} is cooked, reserve the chicken for later use.

> Save the same pan(stainless steel) to brown the vegetables in the next step, in order to get the browned bits released from the bottom of the pan.

== Cooking the Vegetables ==

Add @butter{3%Tbs} to the #&pan(stainless steel)|pan{} and cook the @&(=~2)diced vegetables{} over medium-high heat on a #stove{}, for at least ~{10-15%minutes}, until the vegetables are soft, tender, and the onions start to brown.

Add some @&salt{} and @&black pepper{}. With a #spoon(wooden){1}, scrape up all of the brown bits from the cooked meat which will add intense flavor to the chili.

> Browning equals flavor, so do not rush this step. Note the brown bottom on the pan(stainless steel) when you are finished cooking the meat and vegetables. This is pure flavor! The beer will be used to deglaze the pan, and add this concentrated flavor to the chili.

== Combining and Deglazing ==

Add the @&(=~2)cooked chicken{} and @&(=~3)shredded rotisserie chicken{} and @&(=~1)cooked vegetables{} back to the #&pan(stainless steel)|pan{} and clear a spot in the center of the #&pan(stainless steel)|pan{} to cook the spices for ~{30%seconds}.

Add about @&olive oil{1%Tbs} to the center of the #&pan(stainless steel)|pan{} then add the @&garlic{} and @chili powder{2%Tbs}, @cumin(ground){3%tsp}, and @coriander(ground){1%tsp}. Stir around and cook for about ~{30%seconds}.

> At this point, the bottom of the pan will be very brown with food and spices sticking.

Add @beer(good quality){12%oz} to the #&pan(stainless steel)|pan{} and stir with a #&spoon(wooden){} to deglaze the #&pan(stainless steel)|pan{}, and dissolve all of the brown bits stuck on the bottom.

== Simmering ==

Add @tomato sauce{15%oz can}, @tomatoes(diced){14.5%oz can}, and @Louisiana cayenne hot pepper sauce{1/2%cup}.

Simmer to the desired thickness that you prefer.

Season with @&salt{} and @&black pepper{} to taste. Add additional spices if desired.

== Serving ==

Garnish with your favorite condiments.

> Preferred garnishes include sour cream, chopped green onions, and served with a warm bread stick.

uffalo Chicken Chili
description: >
  A spicy buffalo chicken chili that combines the flavors of buffalo chicken
  wings with traditional chili. This recipe uses two types of chicken meat for
  added flavor and texture, follows traditional Texas-style chili making
  without beans, and creates a thick, savory, and meaty chili with plenty of
  heat. The addition of beer helps deglaze the pan to capture all those flavorful
  brown bits that develop during cooking.
servings: 6
locale: en_US
diet:
  - meat-containing
  - dairy-containing
  - gluten-free
tags:
  - spicy
  - chili
  - buffalo
  - chicken
  - texas-style
cuisine: American
time: 45 minutes
time.prep: 15 minutes
time.cook: 30 minutes
course: main dish
difficulty: moderate
---

> This recipe follows the traditional "Texas" style of chili making and does not include beans. The recipe is very thick, savory, and meaty. One variation is to use two types of chicken meat for added flavor and texture - traditional ground chicken and whole shredded chicken from a store bought rotisserie chicken. If you prefer beans in your chili, feel free to add them (pinto beans would be very good).

> The addition of beer adds some flavor but mostly assists in deglazing the pan which is very important in this recipe. We recommend cooking this chili in a stainless steel pan (do not use a non-stick pan) in order to generate lots of those yummy brown bits (fond) that stick to the bottom of the pan.

== Preparation ==

Starting with a @rotisserie chicken(store-bought){1}, shred the meat from the bones (discarding the skin), and temporarily store the meat in a #bowl{1}.

> If you feel the need to roast your own chicken go for it, but in this case, a store bought roasted chicken saves lots of time and energy. Some people prefer to opt for larger cuts of meat and cut the chicken into cubes rather than shred. We prefer the shredded meat because it adds a thicker texture to the chili, and also adds more surface area to blend with sauce.

Cut @carrots{2}, @celery stalks{3}, @red bell pepper{1}, and @onion(medium){1} into a fine dice. Discard the seeds of the @&red bell pepper{}. Mix in the @?jalapeño peppers(diced){}.

> The addition of diced jalapeño pepper(s) is optional for extra heat. The fine dice adds more surface area to the chili, which adds a more thicker texture, and richer flavor when sautéed.

Mince @garlic{5%cloves}.

== Cooking the Meat ==

Cook the @chicken(ground){1%lb} in a #pan(stainless steel){1} at medium to high heat with a little @olive oil{}. Season the meat with @salt{} and @black pepper{}.

> Make sure the meat gets good and brown (Browning = Flavor). A stainless steel pan is preferred so that the meat will form little browned bits (called fond) that will stick to the bottom of the pan and provide lots of concentrated flavor later on.

When the @&(~1)meat{} is cooked, reserve the chicken for later use.

> Save the same pan(stainless steel) to brown the vegetables in the next step, in order to get the browned bits released from the bottom of the pan.

== Cooking the Vegetables ==

Add @butter{3%Tbs} to the #&pan(stainless steel)|pan{} and cook the @&(=~2)diced vegetables{} over medium-high heat on a #stove{}, for at least ~{10-15%minutes}, until the vegetables are soft, tender, and the onions start to brown.

Add some @&salt{} and @&black pepper{}. With a #spoon(wooden){1}, scrape up all of the brown bits from the cooked meat which will add intense flavor to the chili.

> Browning equals flavor, so do not rush this step. Note the brown bottom on the pan(stainless steel) when you are finished cooking the meat and vegetables. This is pure flavor! The beer will be used to deglaze the pan, and add this concentrated flavor to the chili.

== Combining and Deglazing ==

Add the @&(=~2)cooked chicken{} and @&(=~3)shredded rotisserie chicken{} and @&(=~1)cooked vegetables{} back to the #&pan(stainless steel)|pan{} and clear a spot in the center of the #&pan(stainless steel)|pan{} to cook the spices for ~{30%seconds}.

Add about @&olive oil{1%Tbs} to the center of the #&pan(stainless steel)|pan{} then add the @&garlic{} and @chili powder{2%Tbs}, @cumin(ground){3%tsp}, and @coriander(ground){1%tsp}. Stir around and cook for about ~{30%seconds}.

> At this point, the bottom of the pan will be very brown with food and spices sticking.

Add @beer(good quality){12%oz} to the #&pan(stainless steel)|pan{} and stir with a #&spoon(wooden){} to deglaze the #&pan(stainless steel)|pan{}, and dissolve all of the brown bits stuck on the bottom.

== Simmering ==

Add @tomato sauce{15%oz can}, @tomatoes(diced){14.5%oz can}, and @Louisiana cayenne hot pepper sauce{1/2%cup}.

Simmer to the desired thickness that you prefer.

Season with @&salt{} and @&black pepper{} to taste. Add additional spices if desired.

== Serving ==

Garnish with your favorite condiments.

> Preferred garnishes include sour cream, chopped green onions, and served with a warm bread stick.

## Original

Biscotti are long and hard cookies that many enjoy by dipping into coffee, hot chocolate, or wine. As fancy coffee shops become more and more popular in the United States, biscotti have also become more fashionable (and expensive). It turns out, biscotti is easy to make, and a whole batch costs the same as a single biscotto at Starbucks. Here's my recipe for an Almond and Orange Zest Biscotti that can be enjoyed as is or chocolate dipped.

The name "biscotti" is Italian and literally means twice baked - which is exactly how we'll prepare it. Biscotti can be found in all sorts of flavors, but the most common contain anise, hazelnuts and filberts, and almonds. In this recipe we'll join the flavors of almond and orange (and chocolate). (I should probably also mention that the singular form of biscotti is "biscotto".)

Start by assembling the ingredients: 1 cup (200 g) sugar, 2 large eggs, 3/4 cup (80 g) slivered almonds, 2 tablespoons minced orange zest (about half an orange's zest), 1 teaspoon baking powder, 1/4 teaspoon salt, and 2 cups (250 g) flour. You will also need 1/2 teaspoon (2.5 mL) vanilla extract and 1/4 teaspoon (1.2 mL) almond extract (not shown in photo).


For removing the zest from an orange, I find that when using a Microplane Zester upside down (with the orange under the zester), the zest stays in the device making it much easier to judge how much you've collected. If not using a Microplane (or a zester that produces comparably fine zest), you'll need to mince the zest for this recipe.


Preheat the oven to 350°F (175°C). Whisk the flour, baking powder, and salt together.


Related Articles
Caviar
Basic Biscuits
Shadowbrook Restaurant (Capitola, California)
English Toffee
Chocolate Truffles
Select a mixing bowl that is large enough to hold all the ingredients and still provide enough room to fold them together without making a mess.

Break two large eggs into the large mixing bowl and add the sugar. Whisk to combine.


Continue whisking the sugar and eggs until the color has lightened to a pale yellow. I like using a spare piece of mesh cabinet liner to keep the bowl from shifting while I'm whisking.


Add the vanilla extract, almond extract, orange zest, and slivered almonds to the egg and sugar mixture. Using a spatula, stir once or twice to combine.


Working in batches, pour enough of the flour mixture to cover the surface of the egg mixture. Use a spatula and fold in the flour using as few strokes as possible. Add more flour and fold until all the flour has been integrated. Folding is performed by using a spatula to scoop from either the side or the middle of the mixture and lifting and "folding" (basically movign the spatula laterally and then flipping it over to drop the mixture) onto another part of the mixture. Rotate the bowl each fold.


The key is not to stir or mix the flour with the liquid too much. Gently folding helps prevents the formation of too much elastic gluten. The presence of too much gluten will defeat the delicate and crisp texture we are trying to achieve and result in a possibly chewy product.


Split the batter in half and place the two rough balls onto a non-stick baking sheet (such as a silicone baking mat or parchment paper set in a half sheet pan). With your hands, form the batter into two loaves of approximately 10-in. (25 cm) by 2 in. (5 cm) each. Wetting your hands just a bit may help with molding the loaves since the batter will be fairly sticky.


Bake the loaves at 350°F (175°C) for 40 minutes (rotating the pan once after twenty minutes). The loaves should have just started to crack. (Don't wait for big cracks or you might overcook the biscotti.)

Remove the loaves from the pan and place them on a wire rack to cool for at least ten minutes. This cooling step is extremely important to your non-dominant hand as it will be holding the loaf while you cut it in the next step.


After some cooling, move a loaf to a cutting board and cut diagonally into 3/8-in. (1 cm) thick pieces. Do the same to the other loaf. The interior of each biscotto should still be just a little moist (while the exterior is nice and hard). The crust of the loaf will probably be quite hard, so use a large serrated knife such as a bread knife for this job.


Place the biscotti with a cut side facing up on a half sheet pan and bake for 8 minutes. Remove the pan and flip all the biscotti over so the other cut side is now facing up. Bake for another 7 minutes. Set all the pieces on a wire rack to cool making sure that none of the biscotti are touching each other. If the biscotti are placed too close together, they could get a little soft or soggy as they cool.


Once the biscotti have fully cooled, they can be consumed as is, or chocolate dipped. To chocolate dip, simply break up the chocolate of your choice (dark chocolate is Tina's favorite) and place in a large metal mixing bowl. I find it easier to dip biscotti in large quantities of chocolate, so I usually make two batches of biscotti (4 loaves), and melt about 250 g (a bit more than 1/2 pound) chocolate. For one batch, 125 g (about 1/4 pound) chocolate should be enough. Using a flat bottomed mixing bowl also makes it easier to dip (otherwise you may need to transfer the chocolate to another container after melting to dip the biscotti).


Place the mixing bowl with the chocolate over a saucepan containing about an inch of water (but not so much that the mixing bowl will actually make contact with the water). Bring the water to a boil and then reduce the heat to a simmer. Once the chocolate begins to melt, you can turn off the heat and let the residual heat and steam continue to heat the bowl and melt the chocolate.


Stir occassionally to check when the chocolate has completely melted. Once the chocolate has melted, you can leave the mixing bowl over the hot water to keep the chocolate warm and melted as you dip the biscotti.


Dip each biscotto in the chocolate by inserting the flat bottom into the chocolate. Use a spatula to remove any excess chocolate and then lay the biscotto (chocolate side down) on a silicone mat or sheet of parchment paper. Repeat until all the biscotti have been dipped. The biscotti can then be left to cool on its own or placed in the refrigerator.


The biscotti tastes best during the first few days, but will keep for up to a month in a sealed air-tight container.

## Converted
---
title: Almond and Orange Zest Biscotti
description: >
  Long and hard cookies that many enjoy by dipping into coffee, hot chocolate, 
  or wine. As fancy coffee shops become more and more popular in the United 
  States, biscotti have also become more fashionable (and expensive). It turns 
  out, biscotti is easy to make, and a whole batch costs the same as a single 
  biscotto at Starbucks. This recipe for Almond and Orange Zest Biscotti can be 
  enjoyed as is or chocolate dipped.
servings: 24
locale: en_US
diet:
  - vegetarian
  - nut-containing
  - egg-containing
tags:
  - biscotti
  - italian
  - cookies
  - baking
cuisine: Italian
time: 75 minutes
time.prep: 20 minutes
time.cook: 55 minutes
course: dessert
difficulty: moderate
---

> The name "biscotti" is Italian and literally means twice baked - which is exactly how we'll prepare it. Biscotti can be found in all sorts of flavors, but the most common contain anise, hazelnuts and filberts, and almonds. In this recipe we'll join the flavors of almond and orange (and chocolate). The singular form of biscotti is "biscotto".

== Preparation ==

Preheat the #oven{} to 350°F.

For removing the zest from an @orange{1/2}, use a #Microplane zester{1} upside down (with the orange under the zester), so the zest stays in the device; making it much easier to judge how much you've collected.

> If not using a Microplane (or a zester that produces comparably fine zest), you'll need to mince the zest for this recipe.

== Dry Ingredients ==

#Whisk{1} @all-purpose flour{2%cups}, @baking powder{1%tsp}, and @salt{1/4%tsp} together in a #bowl{1}.

== Wet Ingredients ==

Break @eggs(large){2} into a #mixing bowl(large){1} and add @sugar{1%cup}. #&Whisk{} to combine.

> Select a mixing bowl that is large enough to hold all the ingredients and still provide enough room to fold them together without making a mess.

Continue whisking the @&sugar{} and @&eggs(large){} until the color has lightened to a pale yellow.

> You can use a spare piece of mesh cabinet liner to keep the bowl from shifting while whisking.

Add @vanilla extract{1/2%tsp}, @almond extract{1/4%tsp}, @orange zest(minced){2%Tbs}, and @slivered almonds{3/4%cup} to the @&(~1)egg and sugar mixture{}. Using a #spatula{}, stir once or twice to combine.

== Create the Dough ==

Working in batches, pour enough of the @&(=~2)flour mixture{} to cover the surface of the @&(=~1)egg mixture{}. Use a #&spatula{} and fold in the @&(=~2)flour mixture{} using as few strokes as possible. Add more @&(=~2)flour mixture{} and fold until all the @&(=~2)flour mixture{} has been integrated.

> Folding is performed by using a spatula to scoop from either the side or the middle of the mixture and lifting and "folding" (basically moving the spatula laterally and then flipping it over to drop the mixture) onto another part of the mixture. Rotate the bowl each fold.

> The key is not to stir or mix the flour with the liquid too much. Gently folding helps prevents the formation of too much elastic gluten. The presence of too much gluten will defeat the delicate and crisp texture we are trying to achieve and result in a possibly chewy product.

== First Bake ==

Split the @&(=~1)batter{} in half and place the two rough balls onto a #non-stick baking sheet{} (such as a #?silicone baking mat{} or #?parchment paper{} set in a #?half sheet pan{}).

With your hands, form the @&(=~1)batter{} into two loaves of approximately 10-in. by 2 in. each.

> Wetting your hands just a bit may help with molding the loaves since the batter will be fairly sticky.

Bake the @&(~1)loaves{} at 350°F for ~{40%minutes} (rotating the #&non-stick baking sheet|pan{} once after ~{20%minutes}). The @&(=~2)loaves{} should have just started to crack.

> Don't wait for big cracks or you might overcook the biscotti.

Remove the @&(=~2)loaves{} from the #&non-stick baking sheet|pan{} and place them on a #wire rack{} to cool for at least ~{10%minutes}.

> This cooling step is extremely important to your non-dominant hand as it will be holding the loaf while you cut it in the next step.

== Cutting and Second Bake ==

After some cooling, move a @&(=~1)loaf{} to a #cutting board{} and cut diagonally into 3/8-in. thick pieces with a #serrated knife(large){}. Do the same to the other @&(=~1)loaf{}.

> The interior of each biscotto should still be just a little moist (while the exterior is nice and hard). The crust of the loaf will probably be quite hard, so use a large serrated knife such as a bread knife for this job.

Place the biscotti with a cut side facing up on a #&half sheet pan{} and bake for ~{8%minutes}. Remove the #&half sheet pan|pan{} and flip all the biscotti over so the other cut side is now facing up. Bake for another ~{7%minutes}.

Set all the biscotti on a #&wire rack{} to cool making sure that none are touching each other.

> If the biscotti are placed too close together, they could get a little soft or soggy as they cool.

== Optional: Chocolate Dipping ==

Once the @&(=~1)biscotti{} have fully cooled, they can be consumed as is, or chocolate dipped.

To chocolate dip, break up @?chocolate(dark){125%g} and place in a #mixing bowl(large,metal){1}.

> Dark chocolate is preferred. For easier dipping, use a flat bottomed mixing bowl, otherwise you may need to transfer the chocolate to another container after melting to dip the biscotti.

Place the #&mixing bowl(large,metal)|bowl{} with the @&chocolate(dark){} over a #saucepan{1} with @water{1in}. Bring the @&water{} to a boil and then reduce the heat to a simmer.

Once the @&chocolate(dark){} begins to melt, you can turn off the heat and let the residual heat and steam continue to heat the #&mixing bowl(large,metal)|bowl{} and melt the @&chocolate(dark){}.

> Stir occasionally to check when the chocolate has completely melted. Once the chocolate has melted, you can leave the mixing bowl over the hot water to keep the chocolate warm and melted as you dip the biscotti.

Dip each @&(=~1)biscotto{} in the @&chocolate(dark){} by inserting the flat bottom into the @&chocolate(dark){}. Use a #spatula{} to remove any excess @&chocolate(dark){} and then lay the @&(=~1)biscotto{} (chocolate side down) on a #silicone mat{} or #sheet of parchment paper{}.

Repeat until all the @&(=~1)biscotti{} have been dipped. The @&(=~1)biscotti{} can then be left to cool on its own or placed in the #refrigerator{}.

> The biscotti tastes best during the first few days, but will keep for up to a month in a sealed air-tight container.

## Original
Recipe File
Homemade Mayonnaise by Michael Chu
Recipe Card
Printer-friendly
Normal view
Next »
« Prev
Last night I was making potato salad and tuna salad when I ran out of mayonnaise. I usually have a jar of Best Foods Real Mayonnaise (Best Foods is also known as Hellmann's) in the refrigerator, but this time I forgot to buy some more when I ran low. Although I have made my own mayonnaise in the past (usually for special occasions because homemade mayonnaise is so good), I usually prepare recipes with the store bought variety because it lasts about six months in the refrigerator (while homemade might last up to week). Of all the brands available, I find the Best Foods (or Hellmann's) brand to be the best tasting and most natural (fewest unidentified ingredients) of the supermarket mayonnaises.

Too lazy to leave my home in the middle of food preparation (and too nice of a guy to send Tina on a mayonnaise buying errand), I grabbed a clean bowl and my whisk to make some of my homemade mayo.

All you need are two large egg yolks, 3 tablespoons of lemon juice, 1/4 teaspoon salt, a pinch of white pepper, and 1 cup oil. I ran out of lemon juice last night (I just keep running out of ingredients), so I used about 1 tablespoon lemon juice and 2 tablespoons of lime juice. I also froze the two large egg whites in ice cube trays for later use. For the oil, I used extra light olive oil because of its very faint (almost nonexistant) flavor and nutritional and health properties.


I put the yolks, lemon juice, salt, and pepper into my mixing bowl and whisked until smooth and light. I then whisked the oil, a few drops at a time, into the mixture. I made sure the mixture was smooth and well integrated before pouring the next few drops of oil. The whisking will suspend the oil into the yolk mixture and adding the oil a little at a time will keep the mixture in a state of emulsion - which is what we want.


After about 1/3 cup of oil has been whisked in, you can speed up the pouring a bit. Make sure the mixture is back in emulsion before pouring any more oil. Once all the oil has been whisked in, you have mayonnaise. This is a good time to add any extras, a spoonful of dijon mustard and extra salt and black pepper is usually what I add.


Because handmade mayonnaise is mostly egg yolk, the mayonnaise will have a healthy yellow color. Store bought or machine made mayonnaise usually also contains egg whites which will lighten the color up as well as lighten up the flavor. Anything you don't use immediately, put it in a jar and refrigerate. It should hold for half a week to a week.


Related Articles
Traditional Chicken Pot Pie
Basic Vinaigrette Salad Dressing
Dulce de Leche
Chicago 2011 Part 6 - Noodles by Takashi Yagihashi, Frontera Fresco, Marc Burger
Mini Hamburgers
Homemade Mayonnaise
2 large egg yolks	whisk	whisk oil in drop by drop
3 Tbs. lemon juice
1/4 tsp. salt
pinch of white pepper
1 cup oil
Copyright Michael Chu 2004

You might note that I called both mayonnaise and vinaigrette dressing emulsions. But, a vinaigrette eventually seperates while mayonnaise maintains its state of emulsion. This is because of the egg yolks which contains a substance called lecithin (an emulsifier). You may have seen lecithin as part of the ingredient list of store bought ice cream and salad dressings. This substance when mixed with water (the lemon juice) and oil (the olive oil) helps hold the two together in suspension. Of course, if we kept mixing more and more oil into the mixture, we would eventually overwhelm the emulsifier and the whole emulsion would separate (at least that's what I'm told, maybe one day I'll do it to see what happens when you mix in too much oil). 

## Converted
---
title: Homemade Mayonnaise
author: Michael Chu
description: >
  A creamy, rich homemade mayonnaise that's far superior to store-bought
  varieties. This recipe creates a stable emulsion using egg yolks and oil,
  resulting in a golden-colored mayonnaise with fresh, clean flavors. While
  it requires a bit more effort than opening a jar, the taste difference is
  remarkable and it's perfect for special occasions or when you want the
  freshest ingredients in your potato salad, tuna salad, or sandwiches.
servings: 8
locale: en_US
diet:
  - vegetarian
  - egg-containing
  - dairy-free
tags:
  - condiment
  - sauce
  - homemade
  - emulsion
  - basic
cuisine: European
time: 15 minutes
time.prep: 15 minutes
time.cook: 0 minutes
course: condiment
difficulty: moderate
---

> All you need are basic ingredients to create a superior mayonnaise. The key is patience during the emulsification process - rushing will cause the mixture to break.

== Preparation ==

Place @egg yolks(large){2} in a clean #mixing bowl{1}.

> Store the egg whites in the freezer in ice cube trays for later use.

Add @lemon juice{3%Tbs}, @salt{1/4%tsp}, and @white pepper{1/16%tsp} to the @&egg yolks(large){}.

> If you run out of lemon juice, you can substitute with lime juice - use about 1 tablespoon lemon juice and 2 tablespoons lime juice.

== Create the Base ==

#Whisk{} the @&(=~1)yolk mixture{} until smooth and light.

== Emulsify the Oil ==

While whisking constantly, add @oil(extra light olive){1%cup} to the @&(=~2)yolk mixture{}, a few drops at a time.

> For the oil, extra light olive oil works well because of its very faint flavor and nutritional properties. Make sure the mixture is smooth and well integrated before adding the next few drops of oil.

> The whisking will suspend the oil into the yolk mixture and adding the oil a little at a time will keep the mixture in a state of emulsion - which is what we want.

Once about @&oil(extra light olive){1/3%cup} has been whisked in, speed up the pouring a bit. Make sure the mixture is back in emulsion before pouring any more @&oil(extra light olive){}.

Optionally, add @?dijon mustard{1%tsp}, @?salt{}, and @?black pepper{} to taste.

Continue whisking until all the @&oil(extra light olive){} has been incorporated.

Transfer the @&(~1)mayonnaise{} to a #jar{1} and refrigerate immediately.

> Because handmade mayonnaise is mostly egg yolk, the mayonnaise will have a healthy yellow color. Store bought or machine made mayonnaise usually also contains egg whites which will lighten the color up as well as lighten up the flavor. The mayonnaise should hold for half a week to a week in the refrigerator.

> You might note that mayonnaise is an emulsion that maintains its state unlike vinaigrette dressing which eventually separates. This is because of the egg yolks which contain a substance called lecithin (an emulsifier). This substance when mixed with water (the lemon juice) and oil helps hold the two together in suspension. If we kept mixing more and more oil into the mixture, we would eventually overwhelm the emulsifier and the whole emulsion would separate.

## Original
 Tuna Noodle Casserole by Michael Chu
Recipe Card
Printer-friendly
Normal view
Next »
« Prev
Everyone who makes tuna casserole makes it a different way. There are recipes that use egg noodles (like this one), and there are recipes that use potato chips. Some use a can of cream of mushroom, while others use cream of chicken. The recipe that I like to use starts off with a roux and builds up to a rich and creamy filling of noodles, tuna, and aromatic herbs. The final topping of bread crumbs keeps the top of the casserole from drying out while giving it a pleasant tasting crust.

This recipe is one of the few that survived the "Outlook-Palm Purge". I don't know where I got this recipe, but it seems to be a winner because I don't recall ever getting a complaint when I prepare it.

Start by assembling 12 ounces of light tuna packed in spring water, 1/4 cup all purpose flour, 2-1/2 cup whole milk, 4 oz. sliced button mushrooms, 1/2 cup chopped scallions (also called green onions), 1/4 cup chopped celery, 1 teaspoon dried rosemary, and 1 teaspoon dried thyme. I prefer to use light tuna (usually made with yellowfin and skipjack tuna) for it's milder taste and softer texture than albacore. There will be significantly less than 12 ounces of tuna after we drain the water from the fish, so go ahead and drain the tuna at this point.


Related Articles
The Classic Tiramisu (original recipe?)
Wheat Flour
Traditional Chicken Pot Pie
Simple Tiramisu
Macaroni and Cheese Bake
Get some water boiling when it comes to a boil cook 8 oz. egg noodles according to the directions on the bag. Once the noodles are cooked, remove from the cooking water and rinse with cold water. Set aside.

Melt four tablespoons butter in a saucepan over medium heat.


Add flour to the butter and stir for about two minutes.


The mixture, called a roux, should become smooth and thick during the two minutes.


Add milk slowly to the to the roux while stirring. Continue stirring until the milk thickens. The milk mixture will easily coat the back of your spoon when you dip you spoon in and not flow off quickly when it begins to thicken.


Melt one tablespoon of butter in a skillet.


saute the vegetables until the mushrooms are tender. Then, stir in the rosemary and thyme.


Pour vegetables into the sauce.


Stir until evenly mixed. This is a good time to salt and pepper the sauce. I don't actually measure the amount of salt or pepper that I use for this dish, so I add salt and pepper to taste. Add some salt and pepper, stir, and taste. Repeat as necessary.


Place drained tuna into a large bowl.


Pour noodles and sauce into the bowl containing the tuna and mix until evenly distributed. Pour everything into a greased 8x8 inch baking pan or 1.5 quart casserole.


At this point, you can refrigerate the casserole for up to a couple days without baking it. I cover it with plastic wrap and press the wrap directly onto casserole so no film will form during refrigeration.


When you're ready to serve the casserole, prepare one cup of bread crumbs. I used store bought bread crumbs with herbs in this picture, but I've also used rushed crackers and plain bread crumbs from white bread successfully.


Melt four tablespoons butter in a skillet over medium heat and add the bread crumbs. Stir until the bread crumbs turn a golden brown color.


Sprinkle the bread crumbs over the casserole making sure you cover most of the surface. Any bits of noodle sticking up will dry out and not be tasty, so make sure the noodles are down and covered. Bake at 350°F for thirty minutes (or until the edges begin to bubble).


Cut the casserole after you let it cool for a few minutes.


The casserole is flavorful, but not so complex that you can't taste the individual components. A great hot and hearty dinner for the upcoming autumn and winter.



Tuna noodle casserole (serves four)
Preheat oven to 350°F (175°C)
4 Tbs. (60 g) butter	melt	stir	thicken	mix	season	mix	top	Bake 350°F (175°C) 30 min.
1/4 cup (40 g) all purpose flour	
2-1/2 cup (590 mL) whole milk	
1 Tbs. (15 g) butter	melt	saute	stir
4 oz. (115 g) sliced mushrooms	
1/2 cup (50 g) chopped scallions
1/4 cup (25 g) chopped celery
1 tsp. (1.2 g) dried rosemary	
1 tsp. (1.4 g) dried thyme
salt	
ground black pepper
8 oz.(230 g) egg noodles	cook	
12 oz. (340 g) light tuna	drain
4 Tbs. (60 g) butter	melt	cook	
1 cup (120 g) bread crumbs	
Copyright Michael Chu 2004

## Converted
---
title: Tuna Noodle Casserole
author: Michael Chu
description: >
  Everyone who makes tuna casserole makes it a different way. There are recipes
  that use egg noodles (like this one), and there are recipes that use potato
  chips. Some use a can of cream of mushroom, while others use cream of chicken.
  The recipe that we like to use starts off with a roux and builds up to a rich
  and creamy filling of noodles, tuna, and aromatic herbs. The final topping of
  bread crumbs keeps the top of the casserole from drying out while giving it a
  pleasant tasting crust.
servings: 4
locale: en_US
diet:
  - fish-containing
  - dairy-containing
tags:
  - casserole
  - comfort-food
  - tuna
  - noodles
  - baking
cuisine: American
time: 75 minutes
time.prep: 45 minutes
time.cook: 30 minutes
course: main dish
difficulty: moderate
---

> This recipe is one of the few that survived the "Outlook-Palm Purge". We don't know where this recipe came from, but it seems to be a winner because we don't recall ever getting a complaint when it's prepared.

> We prefer to use light tuna (usually made with yellowfin and skipjack tuna) for its milder taste and softer texture than albacore. There will be significantly less than 12 ounces of tuna after we drain the water from the fish.

Preheat the #oven{} to 350°F.

== Preparation ==

Drain @light tuna(packed in spring water){12%oz} and set aside.

Get some @water(drinkable){} boiling in a #pot(large){1}. When it comes to a boil, cook @egg noodles{8%oz} according to the directions on the bag. Once the @&egg noodles{} are cooked, remove from the cooking water and rinse with cold @&water(drinkable){}. Set aside.

== Make the Milk Sauce ==

Melt @butter{4%Tbs} in a #saucepan{1} over medium heat.

Add @all-purpose flour{1/4%cup} to the @&butter{} and stir for about ~{2%minutes}.

> The mixture, called a roux, should become smooth and thick during the two minutes.

Add @whole milk{2-1/2%cups} slowly to the @&(~1)roux{} while stirring. Continue stirring until the @&whole milk{} thickens.

> The milk mixture will easily coat the back of your spoon when you dip your spoon in and not flow off quickly when it begins to thicken.

== Sauté the Vegetables ==

Melt @butter{1%Tbs} in a #skillet{}.

Sauté @button mushrooms(sliced){4%oz}, @scallions(chopped){1/2%cup}, and @celery(chopped){1/4%cup} until the @&button mushrooms(sliced){} are tender.

Stir in @dried rosemary{1%tsp} and @dried thyme{1%tsp}.

== Combine the Sauce ==

Pour the @&(=~1)sautéed vegetables{} into the @&(=~2)milk sauce{}.

Stir until evenly mixed. Season with @salt{} and @black pepper{} to taste.

> Add some salt and pepper, stir, and taste. Repeat as necessary.

== Assemble the Casserole ==

Place the @&(=~4)drained tuna{} into a #bowl(large){}.

Pour the @&(=~4)cooked noodles{} and @&(=~1)seasoned sauce{} into the #&bowl(large){1} containing the @&(=~4)drained tuna{} and mix until evenly distributed.

Pour everything into a greased #baking pan(8x8in){1} or #casserole dish(1.5 quart){1}.

> At this point, you can refrigerate the casserole for up to a couple days without baking it. Cover it with plastic wrap and press the wrap directly onto casserole so no film will form during refrigeration.

== Prepare the Topping ==

Melt @butter{4%Tbs} in a #&skillet{1} over medium heat and add @bread crumbs{1%cup}.

> Store bought bread crumbs with herbs work well, but you can also use crushed crackers and plain bread crumbs from white bread successfully.

Stir until the @&bread crumbs{} turn a golden brown color.

== Bake ==

Sprinkle the @&(=~1)golden bread crumbs{} over the @&(=~2)casserole{} making sure you cover most of the surface.

> Any bits of noodle sticking up will dry out and not be tasty, so make sure the noodles are down and covered.

Bake at 350°F for ~{30%minutes} (or until the edges begin to bubble).

Let the @&(=~2)casserole{} cool for a few minutes before cutting.

> The casserole is flavorful, but not so complex that you can't taste the individual components. A great hot and hearty dinner for the upcoming autumn and winter.

## General style notes to follow:
Now, part of this conversion is not skimping on a full conversion, taking full advantage of the features of the CookLang specification. As sometimes things like cook time and prep time are lost to history, you're free to make an educated guess on this information and fill it in as metadata. Same for things like the locale (Use the full form -- ISO 639 language code and ISO 3166 alpha2 “country code”), course, difficulty, servings, and cuisine. This is for the modern era, so observe the ingiridents and classify the diets the recipe fits into. Of course, no meats would be vegetarian (for example). Tags are also important! The description should just be a few sentences in length, but should accurately capture the recipe. Use the ">" YAML syntax for appropriate line-breaks. The most important thing is that all of theese fields are filled in, somehow. Don't get lazy! Your job is on the line. For reference, an example conversion is included. You're not exactly trying to be exact, but rather faithful to the original, and making an effort to refrain from any stylistic changes to the recipe, just those that bring it closer to alignment with a semantic cooklang document.

References can only be used once the original ingrident has been established earlier in the text, and annotated as an ingredient.

Cookware can be surmised as anything that assists with the recipe, but is not an actual outcome of the recipe (expected to be eaten). 

Notes are very useful. Their syntax is @&INGRIDENT(NOTE){QUANTITY} Use them to describe attributes of an ingridient. Like walnuts(chopped). They should only be used when the ingredient is generally understood to be easily brought to the state described by its attribute. So you wouldn't do flour(all purpose), as someone purchasing a specializaed flour would not be expected to bring it to an "all purpose" state.

Now when using notes for cookware, understand that quanitity should be used exclusively for the number of a particular item. Notes should be used to describe the shape or size of the item. So if the recipe calls for a bowl, and it doesn't clarify the size, you might have to assume, but an example could be: #bowl(small){2} for two small bowls. This is because size is largely up to interpreation, and notes should be used to provide a semblance of accuracy in ambiguity, not interfere with a recognized constant like the amount of something. For that reason, there's no reason to use language like "pair" when you can define the quantity as two.

Again for cookware, the material would also be defined as a note. So a wooden spoon would be #spoon(wooden){}. The texture is different, so a large serrated knife would be represented as #serrated knife(large){}

It's important to capture ALL of the required cookware, even if the recipe doesn't take the time to explicitly acknolwedge their existence. If for example the recipe says "heat at 350F", it's important to explictly acknolwedge the cookware in the converted recipe.

For references, if the original ingredient/cookware has a note, any future reference need to share it. Or they'll be treated as separate ingredients.

When referencing an original ingredient, be careful with the quantity, as it is treated as cumulative if specified upon a reference. 

Also ensure your vocabulary fits the flow of the recipe, making use of alises to bridge the rigitidity of refrences and the ambiguity of what they might refer to. When examining the tradeoff between accuracy and readability, trust the reader. If the full, complete version has appeared earlier in a step or in previous steps, you have much more leverage in including a simplified, aliased name, as long as the connection is a reasonable jump.

Sections should be used to separate concerns, and break the recipe down into chunks. There is no nesting when it comes to sections. When referencing them as ingredients, there is no specificed naming scheme, use whatever makes sense in the context. Understand that it is common for a section to have more than one ingredient outcome, but is confusing for a step to share that.

For example, a section called "Preperation" might result in chopped vegetables AND peeled potatoes. It's acceptable for these to both be outcomes of a section, while this would be awkward if each outcomes of a single step. Steps should more or less encapsulate a single action.

Use the optional syntax for cookware/ingredients that are truly optional. Any recommendations should not be marked up as actual cookware.

Clarity is very important. Like water should be annotated with a note of (drinkable), or chocolate should be include a note about its cacao density.

All ingredient quantities are lowercase. So no Tbs, only tbs.

Quantities for cookware is just a matter of exclusivitiy. If a recipe requires exclusive access to a particular piece of cookware, than you should mark it as quantity one (or however many of that cookware the recipe needs exclusive access to). Because you think of it in terms of scaling. If you had to make double the recipe, would you need another oven? Probably not. At some point, sure, but in any reasonable case, no. Same logic applies to something like a whisk, or a knife, they can easily switch hands and remain useful across servings. But a bowl? If you have twice the food, you'd need twice the bowls. Or cutting board? 

The > notes as background information are important, and act as tips behind the recipe, guiding the reader. They should be preserved accurately, yet allow the steps to stand alone. 


## Original
 Classic Roast Turkey by Michael Chu
Recipe Card
Printer-friendly
Normal view
Next »
« Prev
The term "classic" is often associated to a minimally seasoned roast turkey. Many people have developed roast turkey recipes that involve cajun spices, honey glazing, lemon infusions, and other techniques that produces a turkey that sets them apart from the classic roast. Since this is our first Thanksgiving together, I thought I would start with the basics and reveal how I roast a turkey.

This recipe is for a 10 to 14 pound turkey. I will update for larger turkeys later. (I rushed this recipe out after roasting a turkey in the wee hours of the morning, so everyone could get a head start on planning for their turkey dinner. I'll correct any mistakes I may have made after the weekend.)

Related Articles
Brining
Thanksgiving Dinner
Turkey or Chicken Stock
Giblet Pan Gravy
Smoked Beer Can Turkey
Before you even think about roasting the turkey, you'll need to budget enough time to thaw, brine, and dry the turkey. If you're purchasing a frozen turkey, allow at least 5 hours per pound of thawing time in the refrigerator. After the turkey has thawed, treat it as if it were fresh (for the purposes of this recipe). Remove the giblets and the neck (found inside the chest cavity). Prepare a brining solution of 1 cup table salt to 1 gallon water in a nonreactive container and soak the turkey in the solution in the refrigerator for four hours. (If your turkey has been infused with a solution, then reduce the salt content in your brine or soak it in a container filled with water.) Pour out the brining solution and rinse the turkey. One convenient way to do this is to position a rack in the sink and place the turkey on the rack to rinse. After the turkey has been rinsed, let it dry by placing it on a rack on a sheet pan in the refrigerator overnight (or for eight hours). Alternatively, use a blow drier on cool setting (no heat) to blow over the skin of the turkey until dry.

Now that the turkey is ready to go, preheat your oven to 400°F. Chop up two medium onions, five carrots, and two celery ribs. Also melt 3 tablespoons butter and set aside 2 tablespoons dried thyme (or two sprigs of fresh thyme). Quantity and even chopping is not that important for this recipe, so feel free to prepare these steps quickly to save time.


From the chopped vegetables, take about half an onion, a carrot, and a half celery rib and combine them with about 1 tsp. thyme and a tablespoon of melted butter. Mix them until evenly distributed.


Throw the prepared vegetables (from the previous step) inside the turkey. Now, tie up the turkey's wings and legs so they will cook evenly. Take a 5 foot (1.5 m) long piece of kitchen twine and tie the drumsticks together as shown.


Loop the twine around the turkey and over the wings.


At the head of the turkey, tie a knot over the flap of skin to hold everything in place.


Place the rest of vegetables and thyme in a roasting pan. If you don't have a roasting pan, you can use a disposable aluminum foil roasting pan from the supermarket. Pour one cup water into the pan and place a V Rack into the pan. Brush breast side of the turkey with butter. Place the turkey on the V rack with the breast side facing down. Brush the back with butter. Place in a 400°F oven.


We're roasting this turkey upside down (usually turkeys are roasted breast up) to cook the breasts at a slower rate. Starting breast side down, gives the legs a head start on cooking. This is desirable because drumsticks and thighs need to be cooked to a higher temperature (about 170°F) in order to remove any trace of pink flesh. The breasts would become very dry and unpalatable if cooked to temperatures as high as the legs.

After 45 minutes, remove the turkey from the oven and baste it with the juices from the roasting pan. I've tried to come up with an easy way to do this without a turkey baster, but I was unable to. Use a turkey baster to reach in between the rungs of the rack and suck up some juices and squirt it over the turkey. Then rotate the turkey onto its side (with a leg sticking up) and brush some more butter on. Return to oven for fifteen more minutes, then baste again and rotate onto other side. Roast for fifteen minutes. Roasting the turkey on its sides lets the sides brown (for better presentation). If you don't care about even browning, you can skip these two rotations and just prolong the breast down roasting by thirty minutes. (You may want to baste once after the 45 minute mark, though.)


Now, rotate the turkey so it is breast side up. Baste again and brush on the remaining butter. Roast for thirty more minutes and then start to check the temperature every ten minutes. The turkey is done when an instant read thermometer thrust into the breast reads 165°F.


The deepest part of the thigh should read between 170°F to 175°F. Remember that the only way to know if your turkey is both safe to eat and not overcooked is with an accurate digital thermometer.


Remove the turkey and allow it to rest for twenty or thirty minutes.


Carving (a quick synopsis)

Place the turkey breast side up on a carving board.

Cut the skin between the thigh and the body of the turkey. Cut in while using a fork to peel the leg away from the body.


Cut through the joint to remove the thigh and drumstick. Place the leg flat on a cutting board. Separate the thigh from the drumstick by cutting through the joint. Cut the meat off the sides of the thigh bone. Cut the meat off the drumstick. Repeat for the other leg.


Remove the wings by pulling them away from the body and thrusting a knife through the joint to sever. Once all the limbs have been removed, cut through the skin along the keel bone.


Angle the blade out a little and cut down along the bone to remove the breast. Do the same to the other side.


Cut the breast meat against the grain into thin slices.


Arrange however you like and serve with those accompaniments that are traditional to your family. (For a nice gravy that goes with this turkey, see Test Recipes: Giblet Pan Gravy.)

Classic Roast Turkey (serves 14)
Prepared Turkey
Preheat oven to 400°F (205°C)
12 lb. turkey, thawed or fresh	brine	stuff	brush on both sides	place on V-rack in pan	roast until breast 165°F (74°C), thigh 170°F (77°C)
1/2 onion, chopped	mix
1 carrot, chopped
1/2 celery rib, chopped
1 tsp. dried thyme
1 Tbs. butter, melted
1 Tbs. butter, melted	
1-1/2 onion, chopped	place in roasting pan
4 carrots, chopped
1-1/2 celery rib, chopped
5 tsp. dried thyme
1 cup water

Roasting table
Time	0:15	0:30	0:45	1:00	1:15	1:30	1:45
Position	breast down	leg up	other leg up	breast up
Action		baste,
rotate,
butter	baste,
rotate,
butter	baste,
rotate,
butter	

## Converted
---
title: Classic Roast Turkey
author: Michael Chu
description: >
  A minimally seasoned roast turkey recipe that focuses on the basics and
  reveals how to roast a turkey properly. The turkey is roasted upside down
  initially to cook the breasts at a slower rate, giving the legs a head start
  on cooking. This technique ensures the legs reach the proper temperature
  while keeping the breast meat moist and prevents the breasts from becoming
  dry and unpalatable.
servings: 14
locale: en_US
diet:
  - meat-containing
  - dairy-containing
tags:
  - turkey
  - roast
  - thanksgiving
  - classic
  - poultry
  - holiday
cuisine: American
time: 6 hours
time.prep: 4 hours
time.cook: 2 hours
course: main dish
difficulty: moderate
---

> The term "classic" is often associated to a minimally seasoned roast turkey. Many people have developed roast turkey recipes that involve cajun spices, honey glazing, lemon infusions, and other techniques that produces a turkey that sets them apart from the classic roast. Since this is our first Thanksgiving together, we thought we would start with the basics and reveal how we roast a turkey.

== Preparation ==

> Before you even think about roasting the turkey, you'll need to budget enough time to thaw, brine, and dry the turkey. If you're purchasing a frozen turkey, allow at least 5 hours per pound of thawing time in the refrigerator.

Remove the giblets and the neck (found inside the chest cavity) from @turkey(thawed or fresh){10-14%lbs}.

Prepare a brining solution by combining @table salt{1%cup} and @water{1%gallon} in a #container(nonreactive,large){1}.

Soak the @&turkey(thawed or fresh){} in the @&(~1)solution{} in the #refrigerator{} for ~{4%hours}.

> If your turkey has been infused with a solution, then reduce the salt content in your brine or soak it in a container filled with water.

Pour out the @&(~1)brining solution{}.

Rinse the @&turkey(thawed or fresh){}.

Position a #rack{1} in the #sink{}.

Place the @&turkey(thawed or fresh){} on the #&rack{} to rinse.

After the @&turkey(thawed or fresh){} has been rinsed, let it dry by placing it on a #&rack{} on a #sheet pan{1} in the #&refrigerator{} overnight (or for ~{8%hours}).

> Alternatively, use a blow drier on cool setting (no heat) to blow over the skin of the turkey until dry.

== Vegetable Preparation ==

Preheat the #oven{} to 400°F.

Chop @onions(medium){2}, @carrots{5}, and @celery ribs{2}.

> Quantity and even chopping is not that important for this recipe, so feel free to prepare these steps quickly to save time.

== Stuffing Preparation ==

Melt @butter{3%tbs}.

From the @&(=~1)chopped vegetables{}, take about @&onions(medium){1/2}, @&carrots{1}, and @&celery ribs{1/2} and combine them with @dried thyme{1%tsp} and @&(~1)melted butter{1%tbs}.

Mix @&(~1)stuffing{} until evenly distributed.

== Turkey Preparation ==

Throw the @&(=~1)prepared vegetable{} inside the @&(=~4)turkey(thawed or fresh){}.

Tie up the @&(=~4)turkey(thawed or fresh){}'s wings and legs so they will cook evenly.

Take some @twine{5%ft} and tie the drumsticks together.

Loop the @&twine{} around the @&(=~4)turkey(thawed or fresh){} and over the wings.

At the head of the @&(=~4)turkey(thawed or fresh){}, tie a knot over the flap of skin to hold everything in place.

== Roasting Setup ==

Place the rest of the @&(=~2)chopped vegetables{} and @&dried thyme{5%tsp} in a #roasting pan{1}.

> If you don't have a roasting pan, you can use a disposable aluminum foil roasting pan from the supermarket.

Pour @water{1%cup} into the #&roasting pan{} and place a #V-rack{1} into the #&roasting pan|pan{}.

Brush breast side of the @&(=~4)turkey(thawed or fresh){} with @&(=4)melted butter{}.

Place the @&(=~4)turkey(thawed or fresh){} on the #&V-rack{} with the breast side facing down.

Brush the back with @&(=4)melted butter{}.

Place in the #&oven{} at 400°F.

> We're roasting this turkey upside down (usually turkeys are roasted breast up) to cook the breasts at a slower rate. Starting breast side down, gives the legs a head start on cooking. This is desirable because drumsticks and thighs need to be cooked to a higher temperature (about 170°F) in order to remove any trace of pink flesh. The breasts would become very dry and unpalatable if cooked to temperatures as high as the legs.

== Roasting Process ==

After ~{45%minutes}, remove the @&(=~4)turkey(thawed or fresh){} from the #&oven{} and baste it with the juices from the #&roasting pan{} using a #turkey baster{1}.

> We've tried to come up with an easy way to do this without a turkey baster, but we were unable to. Use a turkey baster to reach in between the rungs of the rack and suck up some juices and squirt it over the turkey.

Rotate the @&(=~4)turkey(thawed or fresh){} onto its side (with a leg sticking up) and brush some more @&(=4)melted butter{} on.

Return the @&(=~4)turkey(thawed or fresh){} to the #&oven{} for ~{15%minutes}.

Baste again and rotate onto other side.

Roast for ~{15%minutes}.

> Roasting the turkey on its sides lets the sides brown (for better presentation). If you don't care about even browning, you can skip these two rotations and just prolong the breast down roasting by thirty minutes. (You may want to baste once after the 45 minute mark, though.)

Rotate the @&(=~4)turkey(thawed or fresh){} so it is breast side up.

Baste again and brush on the remaining @&(=4)melted butter{}.

Roast for ~{30%minutes} and then start to check the temperature every ~{10%minutes}.

> The @&(=~4)turkey(thawed or fresh){} is considered done when an #instant read thermometer{1} thrust into the breast reads 165°F.

> The deepest part of the thigh should read between 170°F to 175°F.

> Remember that the only way to know if your turkey is both safe to eat and not overcooked is with an accurate digital thermometer.

Remove the @&(=~4)turkey(thawed or fresh){} and allow it to rest for ~{20-30%minutes}.

== Carving ==

Place the @&(=~4)turkey(thawed or fresh){} breast side up on a #carving board{1}.

Cut the skin between the thigh and the body of the @&(=~4)turkey(thawed or fresh){} with a #knife{1}.

Cut in while using a #fork{1} to peel the leg away from the body.

Cut through the joint to remove the thigh and drumstick.

Place the leg flat on a #cutting board{1}. Separate the thigh from the drumstick by cutting through the joint.

Cut the meat off the sides of the thigh bone. Cut the meat off the drumstick. Repeat for the other leg.

Remove the wings by pulling them away from the body and thrusting a #&knife{} through the joint to sever.

Once all the limbs have been removed, cut through the skin along the keel bone.

Angle the blade out a little and cut down along the bone to remove the breast. Do the same to the other side.

Cut the breast meat against the grain into thin slices.

Arrange however you like and serve with those accompaniments that are traditional to your family.

> For a nice gravy that goes with this turkey, see Giblet Pan Gravy.

## Original
Recipe File
Strawberry Glazed Angel Food Cake by Michael Chu
Recipe Card
Printer-friendly
Normal view
Next »
« Prev
What do you do with all those frozen egg whites after making a few batches of creme brulee? Luckily, one of my favorite cakes is the light and fluffy angel food cake. Basically, it's a foam made of egg whites with sugar and flour suspended in it. A tube pan is essential for making this dessert while an electric mixer could be considered optional if you have strong arms and plenty of time. In the past, making an angel food cake was considered difficult because of the time it takes to whisk the egg whites into a workable foam, but with a standing mixer, this recipe is easy to follow and makes for a beautiful and delicious strawberry filled upgrade to the standard angel food cake recipe.

As always, start by assembling the ingredients: 1-1/2 cups cake flour and 1/3 cup granulated sugar (to be sifted together), 1-1/3 cup granulated sugar, 1-1/2 tsp. vanilla extract, 1-1/2 tsp. cream of tartar, 1/4 tsp. salt, and 1-1/2 cup egg whites (about 12 large egg whites). It is important that the egg whites are free of any yolk. The smallest amount of fat (such as from the yolk) may cause the white to not foam up and become a soupy mess. If the egg whites have been previously frozen, thaw them in the refrigerator in a bowl covered in plastic wrap. Then remove them from the fridge and let them warm up to room temperature (about one hour). Room temperature egg whites will produce a foam with larger volume than cold egg whites.


Preheat you oven to 375°F. Sift together the flour and 1/3 cup sugar. I like using a squeeze handle sifter because it sifts the flour into a neat pile and works quickly and efficiently because of its three mesh screens and blades. Sift the flour and sugar at least twice to evenly distribute the sugar within the flour.


Whisk egg whites until they begin to froth. (Whisking in a copper bowl is supposed to produce the most volume, but my Kitchenaid doesn't have a copper bowl and I have no plans to hand whisk my egg whites.)


When the whites start frothing, add the cream of tartar and the salt.


Continue to whisk until the egg whites reach soft peaks. At this point, whisk in the sugar about two tablespoons at a time. While whisking in the sugar bit by bit, add the vanilla extract as well (the exact time you do it doesn't matter). Continue to whisk the egg whites until stiff peaks. (When a foam has reach soft peaks, a whisk dipped into the foam and lifted out, will produce pointed mountains or peaks that droop at the tip. A foam is considered to form stiff peaks when the mountains formed by the lifting whisk do not droop.)


Next, sift a thin layer of the flour mixture over the top of the egg white foam.


Use a spatula to fold the flour into the egg whites, about seven or eight strokes. Don't stir the flour in or over mix or you may collapse some of the egg white foam. The idea is to gently suspend the flour in the foam. Sift more flour onto the foam and continue to fold, repeating until all the flour has been folded in.


Pour the batter into a tube pan and level with a spatula. A tube pan (or tube cake pan) is a special pan that has a center tube that is taller than the sides. This enables the pan to be inverted while cooling. Some pans have a removable bottom which makes cake removal easier. It is also important to keep the tube pan free of any fat. I have a tube pan dedicated to baking angel food cakes to ensure it is free of fat. Bake at 375°F for 35 minutes.


While the cake is baking, make the strawberry glaze. Assemble 8 oz. frozen strawberries, 1/3 cup granulated sugar, 1/2 cup water, 1 Tbs. lemon juice and 1 Tbs. cornstarch.


Combine the sugar, water, lemon juice and strawberries in a saucepan. Stir to dissolve the sugar into the liquid while bringing it up to a boil.


Once the mixture begins to boil, reduce to a simmer and cook for about ten minutes. When the strawberries get soft enough, break them in half with a spoon to help release more flavor from the strawberries.


Remove from then heat and strain the liquid from the strawberries. Press on the solids to squeeze out as much liquid as possible. Set aside the strawberries and return the liquid to the pan.


Bring the liquid back up to a simmer. Whisk 3 tablespoons of water into the cornstarch and then pour it into the simmering liquid.


Increase heat and whisk until the glaze comes to boil. Continue to whisk while the glaze boils and thickens, about 5 minutes. Pour into a bowl and set aside to cool. Once the glaze has cooled down, place it in the refrigerator to chill.


After the cake is done baking, remove it from the oven and immediately invert the pan. Elevating the pan helps allow air to circulate and cool the cake. The can be easily accomplished by setting the pan over a longneck bottle of beer or wine. The neck of the bottle goes into the hole in the center tube and the glass bottle has enough mass to keep the pan from tipping. It is necessary to invert the pan when making angel food cake because the hot cake is in a very delicate state. While cooling, the weight of the cake is enough to collapse it partially. Upside-down, the weight of the cake will help keep the cake tall.


Once the cake has fully cooled (a few hours), run a thin knife around the outside of the cake to separate it from the pan. Also, separate the cake from the center tube. If you have a separating pan, you can remove the outer ring from the base and simply run a knife along the base to free the cake. If you are using a single piece tube pan, pressing into the center a little with the knife while loosening it might help free it from the base.


Remove the cake from the pan and place it on a clean surface.


Using a long serrated knife, cut the cake through the middle (parallel to the counter).


Remove the top layer and dig a shallow furrow in the middle of the bottom layer. I use a teaspoon from my flatware set for this task.


Place the reserved strawberry solids into the furrow.


Return the top of the cake.


Using a spoon or icing spatula, coat the outside of the cake with a layer of strawberry glaze.


Shortly before serving, prepare 1-1/2 cup heavy whipping cream, 1 Tbs. granulated sugar, and 1 tsp. vanilla extract. Using a cold bowl and whisk, whip until the cream reaches stiff peaks. I actually like my whipped cream frosting on angel food cake to be a little over whipped. Continue to whip the cream for a few more seconds and the cream should begin to clump up. I find this "not quite whipped cream, not quite butter" frosting is easier to apply and holds up for longer periods of time without weeping.


Apply whipped cream with an icing spatula on top of the strawberry glaze. I like to cut the angel food cake with a serrated knife (using light pressure while sawing) before applying the whipped cream. This makes it really easy to serve and each individual slice looks beautiful.


Strawberry Glazed Angel Food Cake (serves 12)
Angel Food Cake
Preheat oven to 375°F (190°C)
1-1/2 cup (205 g) cake flour	sift		sift and fold	bake 375°F (190°C) 35 min.
1/3 cup (67 g) granulated sugar
1-1/2 cup (12 large; 360 g) egg whites	whip until frothy	whip until stiff peaks	whip
1-1/2 tsp. cream of tartar	
1/4 (1.5 g) tsp. salt
1-1/3 cup (267 g) sugar	
1-1/2 tsp. (7 mL) vanilla extract

Strawberry glaze
8 oz. (225 g) frozen strawberries	boil	simmer 10 min.	strain	reserve
1/3 cup (67 g) sugar	bring to simmer	whisk & boil until thickened
1/2 cup (120 mL) water
1 Tbs. (15 mL) lemon juice
1 Tbs. (15 mL) cornstarch	whisk
3 Tbs. (45 mL) water

Strawberry Glazed Angel Food Cake
1 angel food cake	cut	stuff	glaze	slice	frost
reserved cooked strawberries	
strawberry glaze	
1-1/3 cup (315 mL) heavy cream	whisk past stiff peaks
1 Tbs. (15 mL) granulated sugar
1 tsp. (5 mL) vanilla extract
Copyright Michael Chu 2004

## Converted
---
title: Strawberry Glazed Angel Food Cake
author: Michael Chu
description: >
  A light and fluffy angel food cake made from egg white foam with sugar and
  flour suspended in it. This beautiful and delicious upgrade to the standard
  angel food cake recipe features a strawberry glaze and whipped cream topping.
  While making angel food cake was once considered difficult due to the time
  required to whisk egg whites, with a standing mixer this recipe is easy to
  follow and produces stunning results.
servings: 12
locale: en_US
diet:
  - vegetarian
  - egg-containing
  - dairy-containing
tags:
  - cake
  - angel-food
  - strawberry
  - dessert
  - baking
  - foam
cuisine: American
time: 3 hours
time.prep: 45 minutes
time.cook: 35 minutes
course: dessert
difficulty: moderate
---

> What do you do with all those frozen egg whites after making a few batches of creme brulee? Luckily, one of our favorite cakes is the light and fluffy angel food cake. Basically, it's a foam made of egg whites with sugar and flour suspended in it. A tube pan is essential for making this dessert while an electric mixer could be considered optional if you have strong arms and plenty of time.

== Angel Food Cake ==

Preheat the #oven{} to 375°F.

> It is important that the egg whites are free of any yolk. The smallest amount of fat (such as from the yolk) may cause the white to not foam up and become a soupy mess. If the egg whites have been previously frozen, thaw them in the refrigerator in a bowl covered in plastic wrap. Then remove them from the fridge and let them warm up to room temperature (about one hour). Room temperature egg whites will produce a foam with larger volume than cold egg whites.

Sift together @cake flour{1-1/2%cups} and @granulated sugar{1/3%cup} using a #sifter{1}.

Sift the @&(~1)flour mixture{} at least twice to evenly distribute the sugar within the flour.

> A squeeze handle sifter works quickly and efficiently because of its three mesh screens and blades, sifting the flour into a neat pile.

Whisk @egg whites{1-1/2%cups} with an #electric mixer{1} until they begin to froth.

> Whisking in a copper bowl is supposed to produce the most volume, but most stand mixers don't have a copper bowl and we have no plans to hand whisk our egg whites.

When the @&egg whites{} start frothing, add @cream of tartar{1-1/2%tsp} and @salt{1/4%tsp}.

Continue to whisk until the @&egg whites{} start to create soft peaks. At this point, whisk in @granulated sugar{1-1/3%cups} about 2tbs at a time.

> When a foam has reached soft peaks, a whisk dipped into the foam and lifted out will produce pointed mountains or peaks that droop at the tip. A foam is considered to form stiff peaks when the mountains formed by the lifting whisk do not droop.

While whisking in the @&granulated sugar{} bit by bit, add @vanilla extract{1-1/2%tsp} as well. Continue to whisk the @&egg whites{} until the @&(~1)cream{} reaches stiff peaks.

Sift a thin layer of the @&(~6)flour mixture{} over the top of the @&(~1)egg white foam{}.

Use a #spatula{1} to fold the @&(~7)flour mixture{} into the @&(~2)egg white foam{}, about seven or eight strokes.

> Don't stir the flour mixture in or over mix or you may collapse some of the egg white foam.

> The idea is to gently suspend the flour in the foam.

Sift more @&(~8)flour mixture{} onto the @&(~3)egg white foam{} and continue to fold, repeating until all the @&(~8)flour mixture{} has been folded in.

Pour the @&(~1)batter{} into a #tube pan{1} and level with a #&spatula{}.

> A tube pan (or tube cake pan) is a special pan that has a center tube that is taller than the sides. This enables the pan to be inverted while cooling. Some pans have a removable bottom which makes cake removal easier. It is also important to keep the tube pan free of any fat. Having a tube pan dedicated to baking angel food cakes helps ensure it is free of fat.

Bake at 375°F for ~{35%minutes}.

== Strawberry Glaze ==

While the @&(=~1)cake{} is baking, make the strawberry glaze.

Combine @frozen strawberries{8%oz}, @granulated sugar{1/3%cup}, @water{1/2%cup}, and @lemon juice{1%tbs} in a #saucepan{1}.

Stir to dissolve the @&granulated sugar{} into the liquid while bringing it up to a boil.

Once the @&(~1)mixture{} begins to boil, reduce to a simmer and cook for about ~{10%minutes}.

When the @&frozen strawberries{} get soft enough, break them in half with a #spoon{1} to help release more flavor from the @&frozen strawberries{}.

Remove from the heat and strain the liquid from the @&frozen strawberries{} using a #strainer{1}.

Press on the solids to squeeze out as much liquid as possible.

Set aside the @&frozen strawberries|cooked strawberries{} and return the liquid to the #&saucepan|pan{}.

Bring the @&(~1)liquid{} back up to a simmer. 

Whisk @water{3%tbs} into @cornstarch{1%tbs} in a #bowl(small){1} and then pour it into the simmering @&(~1)liquid{}.

Increase heat and whisk until the @&(~1)glaze{} comes to boil. Continue to whisk while the @&(~1)glaze{} boils and thickens, about ~{5%minutes}.

Pour into a #bowl{1} and set aside to cool. Once the @&(~1)glaze{} has cooled down, place it in the #refrigerator{} to chill.

== Assembly and Finishing ==

After the @&(=~1)cake{} is done baking, remove it from the #&oven{} and immediately invert the #&tube pan{}.

> Elevating the pan helps allow air to circulate and cool the cake. This can be easily accomplished by setting the pan over a longneck bottle of beer or wine. The neck of the bottle goes into the hole in the center tube and the glass bottle has enough mass to keep the pan from tipping. It is necessary to invert the pan when making angel food cake because the hot cake is in a very delicate state. While cooling, the weight of the cake is enough to collapse it partially. Upside-down, the weight of the cake will help keep the cake tall.

Once the @&(=~1)cake{} has fully cooled (a few hours), run a #knife(thin){1} around the outside of the @&(=~1)cake{} to separate it from the #&tube pan|pan{}.

Also, separate the @&(=~1)cake{} from the center tube. If you have a separating pan, you can remove the outer ring from the base and simply run a #&knife(thin){} along the base to free the @&(=~1)cake{}.

> If you are using a single piece tube pan, pressing into the center a little with the knife while loosening it might help free it from the base.

Remove the @&(=~1)cake{} from the #&tube pan|pan{} and place it on a clean surface.

Using a #serrated knife(long){1}, cut the @&(=~1)cake{} through the middle (parallel to the counter).

Remove the top layer and dig a shallow furrow in the middle of the bottom layer using a #teaspoon{1}.

Place the @&(=~2)cooked strawberries{} into the furrow.

Return the top of the @&(=~1)cake{}.

Using a #spoon{1} or #?icing spatula{1}, coat the outside of the @&(=~1)cake{} with a layer of @&(=~2)strawberry glaze{}.

== Whipped Cream ==

Shortly before serving, prepare @heavy whipping cream{1-1/2%cups}, @granulated sugar{1%tbs}, and @vanilla extract{1%tsp}.

Using a #bowl(cold){1} and #whisk{1}, whip until the @&heavy whipping cream|cream{} reaches stiff peaks.

> We actually like our whipped cream frosting on angel food cake to be a little over whipped. Continue to whip the cream for a few more seconds and the cream should begin to clump up. This "not quite whipped cream, not quite butter" frosting is easier to apply and holds up for longer periods of time without weeping.

Apply @&(~1)whipped cream{} with an #&icing spatula{} on top of the @&(=~2)strawberry glaze{}.

> We like to cut the angel food cake with a serrated knife (using light pressure while sawing) before applying the whipped cream. This makes it really easy to serve and each individual slice looks beautiful.


***AND NOW THE RECIPE YOU WILL CONVERT***
Recipe File
Pumpkin Pie by Michael Chu
Recipe Card
Printer-friendly
Normal view
Next »
« Prev
With Halloween just around the corner and Thanksgiving only a month away, I thought I'd share my recipe for the American tradition called pumpkin pie. Many recipes are fibrous, bland, and either too dry or too wet. Here's a recipe that produces a pumpkin pie that I think is just right.

The tradition of pumpkin pie may date back to the first Thankgiving feast in 1621 (or some say the second Thanksgiving feast). In either case, most likely milk, honey, and spices were poured inside the pumpkin itself. The first pies probably came out more like pudding than like the custard fillings that we use today.

Start by assembling ingredients: 3/4 cup heavy cream, 3/4 cup whole milk, 1 can pumpkin (15 ounces), 1 cup dark brown sugar, 3 large eggs, 1 teaspoon ground cinnamon, 1/2 teaspoon ground ginger, 1/2 teaspoon ground nutmeg, 1/4 teaspoon ground cloves, and 1/2 teaspoon salt. Fresh pumpkin can be used, but it must be cooked first (usually by cutting the pumpkin into pieces and baking in the oven). Its becoming increasingly difficult to find good pumpkins in supermarkets since the ones sold for carving dominate the shelves (or bins) and just aren't that good for eating. Save some time and trouble, buy the canned pumpkin. We'll cook the canned pumpkin briefly with the spices, so it'll be difficult to taste or smell any difference between the canned pumpkin and the fresh for this pie.


Prepare a 9-inch pie crust or defrost a frozen pie crust. Using a fork, punch holes into the dough so it won't rise while prebaking. If using pie weights, this step is unnecessary. Prebake the pie crust according to directions or at 400°F for about 10 minutes.


Meanwhile, whisk pumpkin and spices (cinnamon, ginger, nutmeg, cloves, and salt) together over medium heat in a medium sauce pan. When the pumpkin begins to cook, whisk in the brown sugar.


Once the mixture is fully blended, scrape the sides of the saucepan down and whisk again.


Add milk and cream and continue to whisk.


Once the mixture begins to bubble and splutter, remove from heat.


Place the three eggs into a blender.


Blend the eggs to form a nice creamy consistency (about two seconds).


While blending on low speed, pour the pumpkin pie filling through the feeder hole in the blender cap. This will help break down any fibrous or tough parts of pumpkin creating a smooth filling.


Pour the pumpkin pie filling into the prebaked crust. If you used a deep dish pie crust, this should fit perfectly. I used a "normal" pie crust, so there was about a cup of filling left over, which can be made into pumpkin custards by filling ramekins. Bake at 400°F for 25 minutes or until center of pie is jiggly when pie is rotated gently.


Remove from heat and let cool on a cooling rack for at least one hour. The center will fully set without over cooking the outer edges by removing the pie early. Refrigerate and serve cold, warmed up, or at room temperature.


Related Articles
Wheat Flour
Limeade
Thanksgiving Dinner
Traditional Pecan Pie
Dulce de Leche
Pumpkin Pie (serves 8)
Preheat oven to 400°F (205°C)
15 oz. can (425 g) pumpkin	whisk over medium	whisk over medium	whisk over medium	blend	bake 400°F (205°C) 25 min.
1 tsp. (2 g) ground cinnamon
1/2 tsp. (1 g) ground ginger
1/2 tsp. (1 g) ground nutmeg
1/4 tsp. (0.5 g) ground cloves
1/2 tsp. (3 g) salt
1 cup (200 g) dark brown sugar	
3/4 cup (180 mL) whole milk	
3/4 cup (180 mL) heavy cream
3 large eggs	blend	
1 pie crust	bake blind	
Copyright Michael Chu 2004 
