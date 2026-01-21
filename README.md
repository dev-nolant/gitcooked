# gitcooked.com

> developers have to eat, too

---

## what is this?

a flat, fast recipe website built in rust because i got tired of fancy recipe sites with 47 popups and a video that auto-plays at maximum volume.

just recipes.

## features

- **fast** - rust + axum = instant page loads
- **minimal ui** - inspired by old github
- **markdown export** - because real developers keep their recipes in version control
- **responsive** - works on your phone when you're burning dinner and need instructions NOW
- **github integration** - add recipe via github issues 
- **fun 404 page** - refresh it a few times

## setup

1. clone this thing (repo)
2. `cargo run` (or `cargo build --release`)
3. open `http://127.0.0.1:3000`
4. profit? no, just recipes

### for production

1. create `/recipes/` directory
2. add recipe json files (see format below)
3. set `GITHUB_OWNER` and `GITHUB_REPO` environment variables
4. deploy to netlify/vercel/railway/wherever
5. watch the recipes flow in (now, profit?)

### recipe submission

how it works:
1. user fills out "add recipe" form
2. we redirect them to github with pre-filled issue
3. you review, convert to json file, add to `/recipes/`
4. push to main
5. site rebuilds automatically
6. recipe appears

no tokens. no oauth. no "please authenticate to continue" nonsense.

## recipe format

each recipe is a json file in `/recipes/`. here's what they look like:

```json
{
  "id": "some-uuid-here",
  "title": "Late Night Grilled Cheese",
  "description": "because 2am cravings don't care about nutrition",
  "ingredients": [
    "2 slices whatever bread you have",
    "2 slices cheese (preferably real cheese)",
    "butter. more than you think you need."
  ],
  "instructions": [
    "butter bread like your life depends on it",
    "put cheese between bread slices",
    "fry until golden brown",
    "burn tongue slightly because you couldn't wait 30 seconds"
  ],
  "tags": ["comfort food", "late night", "regret"],
  "created_at": "2024-01-15T02:34:00Z",
  "updated_at": "2024-01-15T02:34:00Z"
}
```

## api

if you're building something on top of this (please do, it's cool):

- `GET /api/recipes` - all the recipes
- `GET /api/recipes/:id` - one specific recipe
- `GET /api/recipes/:id/markdown` - download as md file
- `POST /api/recipes/issue` - generates github issue url for new recipe

## the story

i built this because:

1. modern recipe sites are **terrible**
2. i hate tracking cookies and want to burn them with the heat of a thousand suns
3. sometimes i just want a recipe, not a lifestyle blog and 972 images
4. developers keep recipes as markdown files anyway (right?)
5. i wanted to learn more rust (tick ✅)

the goal: something that loads fast, looks simple, and gets out of your way.

## tech stack

- **rust** - because memory safety is important, even with recipes
- **axum** - web framework (mmmm)
- **tokio** - async runtime
- **serde** - json handling 
- **vanilla javascript** - no frameworks, no build steps, no heavy-weight nonsense

## license

mit. do whatever you want with it. seriously. fork it, deploy it, add features, break it, fix it. it's just recipes. i would just love a lil credit :)

## contributing

found a bug? want a feature? have a recipe to share?

open an issue. or a pr. or both. i'm not your boss. but i will probably merge it

## acknowledgments

- sample recipes generated because empty sites are sad, im not a chef. please don't steal recipes and if you do, clearly note it, so i can at very least credit the writer.
- cooking puns created during moments of questionable decision-making
- the rust community for making async actually pleasant

---

*ps: if you're reading this, you probably should be cooking instead of reading readme files. go make me something tasty.*
