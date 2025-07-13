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

Some recipes are more complex than others and may include components that need to be prepared separately. In such cases, you can use the section syntax, e.g., `==Dough==`. The section name and the `=` symbols after it are optional, and the number of `=` symbols does not matter.

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
automatically enables the [modifiers](#modifiers) extension.

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


Now, part of this conversion is not skimping on a full conversion, taking full advantage of the features of the CookLang specification. As sometimes things like cook time and prep time are lost to history, you're free to make an educated guess on this information and fill it in as metadata. Same for things like the locale (Use the full form -- ISO 639 language code and ISO 3166 alpha2 “country code”), course, difficulty, servings, and cuisine. This is for the modern era, so observe the ingiridents and classify the diets the recipe fits into. Of course, no meats would be vegetarian (for example). Tags are also important! The description should just be a few sentences in length, but should accurately capture the recipe. Use the ">" YAML syntax for appropriate line-breaks. The most important thing is that all of theese fields are filled in, somehow. Don't get lazy! Your job is on the line. For reference, an example conversion is included. You're not exactly trying to be exact, but rather faithful to the original.

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
  ranging from moist to cakey. Personally, I like banana nut bread that is
  denser than a classic yellow cake, but not quite as dry as wheat bread. The
  banana nut bread should be tender and flavorful, but not have the consumer
  feel like he needs to drink a glass of water with it. Some of the recipes
  that I've tried were so moist that the bread sticks to the roof of the mouth,
  while other recipes were much too dry - both require drinking a glass of
  water to get the bread down. (Of course, drinking a glass of milk while
  eating a slice of good banana bread is an awesome combination, but it
  shouldn't be considered a necessity for enjoying banana nut bread.) This
  recipe results in what I feel is the perfect combination of flavor and
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

Start by preparing a #loaf pan{5inx9in} by buttering the bottom and sides. Lightly flour the #&loaf pan{} and tap out the excess flour.

Combine and whisk all the dry ingredients except for the walnuts: @all-purpose flour{1-1/3%cups}, @sugar{2/3%cup}, @baking soda{1/2%tsp}, @baking powder{1/4%tsp}, and @salt{1/2%tsp}.

> The use of both baking soda and powder are necessary to provide enough leavening for the proper consistency of the bread. The baking soda is just enough to utilize the slight acidity of the bananas to create the desired carbon dioxide bubbles. Baking powder (which is a mix of baking soda, a base, and cream of tartar, an acid) provides even more leavening power.

== Create the Banana Mixture == 
Mash @bananas(ripe){2%large}, @butter(melted){6%Tbs}, and @vanilla extract{1%tsp} together. Lightly beat @eggs(large){2} together.

== Create the Batter == 

Mash the @&(=2)banana mixture{} with the @eggs(large){2} until smooth and well blended.

Pour the @&(=2)banana mixture{} onto the dry ingredients. Add @walnuts(chopped){1/2%cup}.

Fold together until no more white flour is uncovered while folding.

== Bake ==

Pour the @&(=3)batter into the prepared #&loaf pan{} and bake for ~{55%minutes} at 350ºF.

== Serve ==

After ~{55%minutes}, the loaf of banana bread should be about done. Set the pan on the #wire rack{} to cool for ~{10%minutes}.

> You can know if the bread is cooked through when a wooden toothpick inserted into the center comes out clean.

Remove the loaf from the #&loaf pan{} and let cool on the #&wire rack{}. Serve warm or fully cooled.

> The loaf can be wrapped in plastic and stored at room temperature for about four or five days.

***And finally, the recipe you're tasked with converting***
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


