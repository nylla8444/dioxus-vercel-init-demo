# dioxus-vercel init demo

A [Dioxus](https://dioxuslabs.com) web app that [Vercel](https://vercel.com) builds by itself on every push to this repo, set up with [dioxus-vercel](https://github.com/nylla8444/dioxus-vercel)'s `init` command.

The whole setup is this `vercel.json`:

```json
{
  "framework": null,
  "installCommand": "cargo install dioxus-vercel --version 0.3.0 --locked",
  "buildCommand": "dioxus-vercel build"
}
```

The repo is connected to Vercel with **Add New > Project > Import**. There's no token, secret or GitHub Actions workflow, and nothing is built locally. Pull requests get Vercel's own preview deployments.

To do the same with your app, see [Let Vercel build it](https://github.com/nylla8444/dioxus-vercel#let-vercel-build-it).
