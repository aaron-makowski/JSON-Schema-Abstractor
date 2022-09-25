# RUST based schema abstraction

## What it does
tl;dr: You feed in a bunch of JSON and it abstracts a schema from it and can validate data against that schema.

- Finds the optional and mandatory fields
- Determines field types/range of types
- Stores Master Schema to validate data against in the future

This project was my first RUST endeavor and I learned a lot of low level concepts that I missed out on by learning C++/JAVA in highschool rather than delving into C at all.

I have included the /project context/ folder, which is the abstractor in the context of the actual OpenFaaS serverless function (`.toml`s, etc)
`libs.rs` & `schema_gen.rs` are the main code files and included at the top level for convenience
