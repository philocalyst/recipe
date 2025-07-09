import { commentRegex, tokenRegex } from "/recipes/js/regex.js";// /recipes/js/regex.js
import { text, ingredient, cookware, timer, } from "/recipes/js/token.js";// /recipes/js/token.js
const parseQuantity = (quantity, fallback) => {
    if (quantity.includes("/")) {
        const [a, b] = quantity.split("/", 2);
        return Number(a) / Number(b);
    }
    if (Number(quantity)) {
        return Number(quantity);
    }
    else if (quantity.trim() === "") {
        return fallback;
    }
    else {
        return quantity.trim();
    }
};
export const parse = (source) => {
    const steps = [];
    const metadata = {};
    const sourceWithoutComments = source.replace(commentRegex, "");
    const lines = sourceWithoutComments.split(/\r?\n|\r/); ///had to change to split on only \r
    for (const line of lines) {
        const step = [];
        let position = 0;
        const matches = line.matchAll(tokenRegex);
        for (const match of matches) {
            if (match.index === undefined)
                continue;
            if (match.groups === undefined)
                continue;
            const groups = match.groups;
            if (position < match.index) {
                step.push(text(line.substring(position, match.index)));
            }
            // metadata
            if (groups.metaKey) {
                const { metaKey, metaValue } = groups;
                metadata[metaKey.trim()] = metaValue === null || metaValue === void 0 ? void 0 : metaValue.trim();
            }
            // ingredients
            else if (groups.ingredientName) {
                const { ingredientName } = groups;
                step.push(ingredient(ingredientName, "some", ""));
            }
            else if (groups.ingredientName_) {
                const { ingredientName_, ingredientQuantity = "" } = groups;
                const [quantityRaw, units = ""] = ingredientQuantity.split("%", 2);
                const quantity = parseQuantity(quantityRaw, "some");
                step.push(ingredient(ingredientName_, quantity, units.trim()));
            }
            // cookware
            else if (groups.cookwareName) {
                const { cookwareName } = groups;
                step.push(cookware(cookwareName, ""));
            }
            else if (groups.cookwareName_) {
                const { cookwareName_, cookwareQuantity = "" } = groups;
                step.push(cookware(cookwareName_, parseQuantity(cookwareQuantity, "")));
            }
            // timer
            else if (groups.timerName !== undefined) {
                const { timerName, timerQuantity = "" } = groups;
                const [quantityRaw, units = ""] = timerQuantity.split("%", 2);
                const quantity = parseQuantity(quantityRaw, "");
                step.push(timer(timerName, quantity, units.trim()));
            }
            position = match.index + match[0].length;
        }
        if (position < line.length) {
            step.push(text(line.substring(position)));
        }
        if (step.length > 0) {
            steps.push(step);
        }
    }
    return { steps, metadata };
};
export const Recipe = (recipe) => {
    const { steps, metadata } = parse(recipe);
    const ingredients = [];
    const cookware = [];
    for (const step of steps) {
        for (const token of step) {
            if (token.type === "ingredient") {
                ingredients.push(token);
            }
            else if (token.type === "cookware") {
                cookware.push(token);
            }
        }
    }
    return {
        metadata,
        ingredients,
        cookware,
        steps,
    };
};
