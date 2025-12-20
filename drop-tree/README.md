# drop-tree

> [!CAUTION]
> This crate is experimental, use with caution.

`drop-tree` is an experimental Rust crate for modelling structured ownership in FFI code.

It provides a low-cost runtime abstraction over ownership rules that makes sure resources are destroyed in the correct order, without requiring explicit lifetimes.

This crate...

- models ownership relationships at runtime
- guarantees parents are dropped *after* all children
- avoids explicit lifetimes in consumer code

This crate does not...

- enforce Rust's borrowing rules
- prevent aliasing or mutable aliasing
- replace lifetimes entirely

**Borrowing correctness is still the responsibility of the dowstream crate.**

If your design can be expressed cleanly using normal explicit lifetimes, you probably do not need this crate.

## Motivation

In Rust, lifetimes model both borrowing *and* ownership. This is usually a sensible idea but in certain situations, such as FFI code, it can be somewhat of a hindrance.

FFI code may often contain multiple structures that "borrow" from one another while still needing to coexist and act independently (for example, being stored on the same struct). Expressing these relationships with lifetimes can force everything to be created in the same scope or location and/or have the borrow checker reject valid designs due to overly strict borrowing constraints. 
Take, for example, two structs `App` and `Context`, where a `Context` is created and owned by an `App`, and logically borrows from the `App`. This could be modelled like:

```rs
struct App<'app> {
    // ...
    _phantom: PhantomData<&'app ()>
}

struct Context<'app> {
    // ...
    _phantom: PhantomData<&'app ()>
}

impl<'app> App<'app> {
    fn new_context(&'app self) -> Context<'app> {
        Context { _phantom: PhantomData }
    }
}

////

struct MyApp<'app> {
    app: App<'app>,
    context: Context<'app>
}
```
However, trying to construct `MyApp` would probably result in an error along the lines of:

```
error[E0505]: cannot move out of `...` because it is borrowed
```

The code above is not wrong but it doesn't really make sense to be modelling borrowing rules here. Borrowing rules are often modelled at the API boundary and not on the internal structures. In the code above, the implementation only needs to guarantee correct resource lifetimes; access rules would be enforced by the methods implemented on each structure respectively.

This is what `drop-tree` is used for. This crate does not attempt to enforce borrowing rules (that responsibility is delegated to downstream code). Instead, its purpose is to provide an ownership structure that guarantees resources are destroyed in the correct order.

The same code above modelled using `drop-tree` would look like:

```rs
#[drop_tree]
struct App {
    // ...
}

#[drop_tree(borrows(App))]
struct Context {
    // ...
}
```

This keeps `App` alive while any `Context`'s are still alive.

## Important Notes

In order to prevent the data of a node being dropped, the actual generated code expands to roughly:

```rs
// #[drop_tree]
// struct App {
//     user_field: ()
// }

struct AppMarker;

struct AppData {
    user_field: ()
}

struct App {
    _links: OwnershipLinks<ErasedOwnershipHandle<AppMarker>, ...>
}
```

This allows, in this instance, `AppData` to *only* be dropped when the dependant resources have already been dropped. However, this **does not** prevent the `App` struct itself from being dropped. The macro automatically implements `Drop` for this struct to prevent the user from incorrectly implementing `Drop` (in which they may incorrectly do cleanup, causing UB). This is what the `destructor` argument is used for, as that is only called when the inner data struct is dropped.

## Design rationale

### Type erasure

Internally, `drop-tree` uses type-erased ownership handles.

This is because:

- parent nodes may be generic (e.g. `Foo<B>`)
- child nodes should not need to carry those generics
- propagating generics through the entire ownership tree is undesireable
  
This is where most of the unsafe code is, as it requires casting from the conrete type to the type-erased version and later casting back from type-erased to concrete. The use of trait bounds and marker types *should* prevent any unsound casting, however I haven't 100% validated this yet.

### Generated code

`drop-tree` uses an attribute macro as it requires modifying changing the struct fields directly. The macro also generates a rust module with extra structs contained within. Some of these structs need to be made public for other invocations of the macro, such as when a node borrows from another node. Unfortunately, this leads to pollution of the intellisense autocomplete recommendations. I have applied `#[doc(hidden)]` where I can, but they still show up. This is something in the TODO list I would like to fix.

## TODO

- [ ] Validate unsafe code for soundness
- [ ] Add an thread-safe version of the tree
- [ ] Clean up the proc-macro implementation
- [ ] Reduce the visibility of certain generated types in intellisense (while keeping public for macro expansion)


## License

MIT License