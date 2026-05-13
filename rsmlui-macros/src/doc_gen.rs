use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, RwLock};

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{ToTokens, quote};
use regex::Regex;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Attribute, Expr, ExprLit, Fields, ImplItem, ItemEnum, ItemImpl, ItemMod, ItemStruct, ItemTrait,
    ItemType, Lit, Meta, MetaNameValue, Token, TraitItem, Visibility, parse_macro_input,
    parse_quote,
};

static DOC_CACHE: LazyLock<RwLock<HashMap<PathBuf, ParsedDocFile>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

static DOC_PARSE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?s)<!--\s*rsmlui:block\["(?P<item>[^"]+)"\]\s+refid="(?P<refid>[^"]*)"\s*-->\s*(?P<content>.*?)\s*<!--\s*/rsmlui:block\s*-->"#,
    ).unwrap()
});

#[derive(Debug, Clone)]
struct ParsedDocFile {
    by_name: HashMap<String, Vec<(String, String)>>,
    by_refid: HashMap<String, String>,
}

fn parse_rsmlui_markdown(contents: &str, error_span: Span) -> syn::Result<ParsedDocFile> {
    let mut parsed_doc_file = ParsedDocFile {
        by_name: HashMap::new(),
        by_refid: HashMap::new(),
    };

    for capture in DOC_PARSE_REGEX.captures_iter(contents) {
        let item_name = capture
            .name("item")
            .ok_or_else(|| syn::Error::new(error_span, "could not find capture group `item`"))?
            .as_str();
        let refid = capture
            .name("refid")
            .ok_or_else(|| syn::Error::new(error_span, "could not find capture group `refid`"))?
            .as_str();
        let content = capture
            .name("content")
            .ok_or_else(|| syn::Error::new(error_span, "could not find capture group `contents`"))?
            .as_str();

        parsed_doc_file
            .by_name
            .entry(item_name.to_owned())
            .or_default()
            .push((refid.to_owned(), content.to_owned()));

        if !refid.is_empty() {
            parsed_doc_file
                .by_refid
                .insert(refid.to_owned(), content.to_owned());
        }
    }

    Ok(parsed_doc_file)
}

fn load_doc_file(path: &Path, error_span: Span) -> syn::Result<ParsedDocFile> {
    {
        let cache = DOC_CACHE.read().unwrap();
        if let Some(parsed) = cache.get(path) {
            return Ok(parsed.clone());
        }
    }

    let markdown = fs::read_to_string(path)
        .map_err(|_| syn::Error::new(error_span, "file could not be found"))?;

    let parsed = parse_rsmlui_markdown(&markdown, error_span)?;

    let mut cache = DOC_CACHE.write().unwrap();
    cache.insert(path.to_owned(), parsed.clone());

    Ok(parsed)
}

fn get_item_doc_from_file(
    doc_file: &ParsedDocFile,
    item: &str,
    refid: Option<&str>,
    error_span: Span,
) -> syn::Result<String> {
    if let Some(refid) = refid {
        return doc_file
            .by_refid
            .get(refid)
            .cloned()
            .ok_or_else(|| syn::Error::new(error_span, "item with refid could not be found"));
    }

    match doc_file.by_name.get(item) {
        Some(blocks) => match blocks.as_slice() {
            [(_, content)] => Ok(content.clone()),
            [] => Err(syn::Error::new(error_span, "item could not be found")),
            _ => Err(syn::Error::new(
                error_span,
                "multiple overloads exist for this item; use `refid` to disambiguate",
            )),
        },
        None => Err(syn::Error::new(error_span, "item could not be found")),
    }
}

enum Documentable {
    Struct(ItemStruct),
    Enum(ItemEnum),
    Type(ItemType),
    Impl(ItemImpl),
    Trait(ItemTrait),
    Mod(ItemMod),
}

impl Parse for Documentable {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let forked = input.fork();

        forked.call(Attribute::parse_outer)?;
        forked.parse::<Visibility>()?;

        let item_type = forked.lookahead1();

        if item_type.peek(Token![struct]) {
            let item = input.parse::<ItemStruct>()?;

            Ok(Self::Struct(item))
        } else if item_type.peek(Token![enum]) {
            let item = input.parse::<ItemEnum>()?;

            Ok(Self::Enum(item))
        } else if item_type.peek(Token![type]) {
            let item = input.parse::<ItemType>()?;

            Ok(Self::Type(item))
        } else if item_type.peek(Token![trait]) {
            let item = input.parse::<ItemTrait>()?;

            Ok(Self::Trait(item))
        } else if item_type.peek(Token![impl]) {
            let item = input.parse::<ItemImpl>()?;

            Ok(Self::Impl(item))
        } else if item_type.peek(Token![mod]) {
            let item = input.parse::<ItemMod>()?;

            Ok(Self::Mod(item))
        } else {
            Err(input.error("expected struct, enum, type, module, or impl"))
        }
    }
}

impl ToTokens for Documentable {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(match self {
            Documentable::Struct(item_struct) => quote! { #item_struct},
            Documentable::Enum(item_enum) => quote! { #item_enum},
            Documentable::Type(item_type) => quote! { #item_type},
            Documentable::Trait(item_trait) => quote! { #item_trait},
            Documentable::Impl(item_impl) => quote! { #item_impl},
            Documentable::Mod(item_mod) => quote! { #item_mod},
        });
    }
}

#[derive(Debug)]
struct ContainerDocArguments {
    file: (String, Span),
    name: Option<(String, Span)>,
    refid: Option<(String, Span)>,
}

impl Parse for ContainerDocArguments {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let arguments: Punctuated<MetaNameValue, Token![,]> = Punctuated::parse_terminated(input)?;

        let mut file = None;
        let mut name = None;
        let mut refid = None;

        for argument in arguments.iter() {
            if argument.path.is_ident("file") {
                if file.is_none() {
                    match &argument.value {
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(string),
                            ..
                        }) => {
                            file = Some((string.value(), argument.span()));
                        },
                        _ => {
                            return Err(syn::Error::new(
                                argument.span(),
                                "`file` argument must be a string",
                            ));
                        },
                    }
                } else {
                    return Err(syn::Error::new(
                        argument.span(),
                        "`file` argument must only exist once",
                    ));
                }
            } else if argument.path.is_ident("name") {
                if name.is_none() {
                    match &argument.value {
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(string),
                            ..
                        }) => {
                            name = Some((string.value(), argument.span()));
                        },
                        _ => {
                            return Err(syn::Error::new(
                                argument.span(),
                                "`name` argument must be a string",
                            ));
                        },
                    }
                } else {
                    return Err(syn::Error::new(
                        argument.span(),
                        "`name` argument must only exist once",
                    ));
                }
            } else if argument.path.is_ident("refid") {
                if refid.is_none() {
                    match &argument.value {
                        Expr::Lit(ExprLit {
                            lit: Lit::Str(string),
                            ..
                        }) => {
                            refid = Some((string.value(), argument.span()));
                        },
                        _ => {
                            return Err(syn::Error::new(
                                argument.span(),
                                "`refid` argument must be a string",
                            ));
                        },
                    }
                } else {
                    return Err(syn::Error::new(
                        argument.span(),
                        "`refid` argument must only exist once",
                    ));
                }
            } else {
                return Err(syn::Error::new(
                    argument.span(),
                    "unexpected field (expected only `name`, `refid`, and/or `file`)",
                ));
            }
        }

        if file.is_none() {
            return Err(syn::Error::new(input.span(), "`file` argument is required"));
        }

        Ok(Self {
            file: file.unwrap(),
            name,
            refid,
        })
    }
}

fn make_doc_attributes<'a, A: Iterator<Item = &'a Attribute> + Clone>(
    attrs: A,
    doc_file: &ParsedDocFile,
    name: &str,
    refid: Option<&str>,
    span: Span,
    module: bool,
) -> syn::Result<Vec<Attribute>> {
    let has_other_doc_comments = attrs.clone().any(|attr| attr.path().is_ident("doc"));

    let mut new_attrs = vec![];

    let section = get_item_doc_from_file(doc_file, name, refid, span)?;

    if has_other_doc_comments {
        if module {
            new_attrs.push(parse_quote! {
                #![doc = "# Notes"]
            });
        } else {
            new_attrs.push(parse_quote! {
                #[doc = "# Notes"]
            });
        }
    }

    new_attrs.extend(attrs.cloned());

    if module {
        new_attrs.push(parse_quote! {
            #![doc = "# RmlUi Documentation"]
        });
        new_attrs.push(parse_quote! {
            #![doc = #section]
        });
    } else {
        new_attrs.push(parse_quote! {
            #[doc = "# RmlUi Documentation"]
        });
        new_attrs.push(parse_quote! {
            #[doc = #section]
        });
    }

    Ok(new_attrs)
}

fn remove_rmldoc_attrs(
    attrs: &[Attribute],
) -> syn::Result<impl Iterator<Item = &'_ Attribute> + Clone> {
    Ok(attrs
        .iter()
        .filter(|attr| !attr.path().is_ident("rmldoc")))
}

fn parse_replace_rmldoc_attr(
    doc_file: &ParsedDocFile,
    attrs: &[Attribute],
) -> syn::Result<Vec<Attribute>> {
    let mut name = None;
    let mut name_span = None;
    let mut refid = None;

    for attr in attrs {
        match &attr.meta {
            Meta::List(list) => {
                if list.path.is_ident("rmldoc") {
                    let arguments: Punctuated<MetaNameValue, Token![,]> =
                        Punctuated::parse_terminated.parse2(list.tokens.clone())?;

                    for argument in arguments.iter() {
                        if argument.path.is_ident("name") {
                            if name.is_none() {
                                match &argument.value {
                                    Expr::Lit(ExprLit {
                                        lit: Lit::Str(string),
                                        ..
                                    }) => {
                                        name = Some(string.value());
                                        name_span = Some(attr.span());
                                    },
                                    _ => {
                                        return Err(syn::Error::new(
                                            argument.span(),
                                            "`name` argument must be a string",
                                        ));
                                    },
                                }
                            } else {
                                return Err(syn::Error::new(
                                    attr.span(),
                                    "field must only have one `name` argument",
                                ));
                            }
                        } else if argument.path.is_ident("refid") {
                            if refid.is_none() {
                                match &argument.value {
                                    Expr::Lit(ExprLit {
                                        lit: Lit::Str(string),
                                        ..
                                    }) => {
                                        refid = Some(string.value());
                                    },
                                    _ => {
                                        return Err(syn::Error::new(
                                            argument.span(),
                                            "`refid` argument must be a string",
                                        ));
                                    },
                                }
                            } else {
                                return Err(syn::Error::new(
                                    attr.span(),
                                    "field must only have one `refid` argument",
                                ));
                            }
                        } else {
                            return Err(syn::Error::new(
                                argument.span(),
                                "unexpected field (expected only `name` and/or `refid`)",
                            ));
                        }
                    }
                }
            },
            _ => continue,
        }
    }

    let filtered_attrs = remove_rmldoc_attrs(attrs)?;

    let new_attrs;

    if let (Some(name), Some(name_span)) = (name, name_span) {
        let refid = refid.as_deref();
        new_attrs = make_doc_attributes(filtered_attrs, doc_file, &name, refid, name_span, false)?;
    } else {
        new_attrs = filtered_attrs.cloned().collect::<Vec<_>>();
    }

    Ok(new_attrs)
}

fn parse_doc_file(file_name: &str, file_span: Span) -> syn::Result<ParsedDocFile> {
    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");

    let doc_file = workspace_dir.join("rml-doc/generated/md").join(file_name);

    load_doc_file(&doc_file, file_span)
}

fn make_doc_comments(
    input: &mut Documentable,
    arguments: &ContainerDocArguments,
    arguments_span: Span,
) -> syn::Result<TokenStream2> {
    let (ref file_name, file_span) = arguments.file;

    let parsed_doc_file = parse_doc_file(file_name, file_span)?;

    match input {
        Documentable::Struct(item_struct) => {
            let ItemStruct {
                attrs,
                vis,
                struct_token,
                ident,
                generics,
                fields,
                semi_token,
            } = item_struct;

            let container_attrs = if let Some((ref name, name_span)) = arguments.name {
                let refid = arguments.refid.as_ref().map(|(s, _)| s.as_str());
                make_doc_attributes(attrs.iter(), &parsed_doc_file, name, refid, name_span, false)?
            } else {
                vec![]
            };

            match fields {
                Fields::Named(fields_named) => {
                    for named in &mut fields_named.named {
                        named.attrs = parse_replace_rmldoc_attr(&parsed_doc_file, &named.attrs)?;
                    }
                },
                Fields::Unnamed(fields_unnamed) => {
                    for unnamed in &mut fields_unnamed.unnamed {
                        unnamed.attrs =
                            parse_replace_rmldoc_attr(&parsed_doc_file, &unnamed.attrs)?;
                    }
                },
                _ => {},
            }

            let where_clause = &generics.where_clause;

            Ok(quote! {
                #(#container_attrs)*
                #vis #struct_token #ident #generics #where_clause #fields #semi_token
            })
        },
        Documentable::Enum(item_enum) => {
            let ItemEnum {
                attrs,
                vis,
                enum_token,
                ident,
                generics,
                brace_token,
                variants,
            } = item_enum;

            let container_attrs = if let Some((ref name, name_span)) = arguments.name {
                let refid = arguments.refid.as_ref().map(|(s, _)| s.as_str());
                make_doc_attributes(attrs.iter(), &parsed_doc_file, name, refid, name_span, false)?
            } else {
                vec![]
            };

            for variant in variants.iter_mut() {
                variant.attrs = parse_replace_rmldoc_attr(&parsed_doc_file, &variant.attrs)?;
            }

            let where_clause = &generics.where_clause;

            let mut block = quote! {};

            brace_token.surround(&mut block, |block| {
                block.extend(quote! { #variants });
            });

            Ok(quote! {
                #(#container_attrs)*
                #vis #enum_token #ident #generics #where_clause #block
            })
        },
        Documentable::Trait(item_trait) => {
            let ItemTrait {
                attrs,
                vis,
                unsafety,
                auto_token,
                trait_token,
                ident,
                generics,
                colon_token,
                supertraits,
                brace_token,
                items,
                ..
            } = item_trait;

            let container_attrs = if let Some((ref name, name_span)) = arguments.name {
                let refid = arguments.refid.as_ref().map(|(s, _)| s.as_str());
                make_doc_attributes(attrs.iter(), &parsed_doc_file, name, refid, name_span, false)?
            } else {
                vec![]
            };

            for item in items.iter_mut() {
                match item {
                    TraitItem::Const(trait_item_const) => {
                        trait_item_const.attrs =
                            parse_replace_rmldoc_attr(&parsed_doc_file, &trait_item_const.attrs)?;
                    },
                    TraitItem::Fn(trait_item_fn) => {
                        trait_item_fn.attrs =
                            parse_replace_rmldoc_attr(&parsed_doc_file, &trait_item_fn.attrs)?;
                    },
                    TraitItem::Type(trait_item_type) => {
                        trait_item_type.attrs =
                            parse_replace_rmldoc_attr(&parsed_doc_file, &trait_item_type.attrs)?;
                    },
                    _ => {},
                }
            }

            let where_clause = &generics.where_clause;

            let mut block = quote! {};

            brace_token.surround(&mut block, |block| {
                block.extend(quote! { #(#items)* });
            });

            Ok(quote! {
                #(#container_attrs)*
                #vis #unsafety #auto_token #trait_token #ident #generics #colon_token #supertraits #where_clause #block
            })
        },
        Documentable::Impl(item_impl) => {
            let ItemImpl {
                items,
                attrs,
                defaultness,
                unsafety,
                impl_token,
                generics,
                trait_,
                self_ty,
                brace_token,
            } = item_impl;

            for item in items.iter_mut() {
                match item {
                    ImplItem::Const(impl_item_const) => {
                        impl_item_const.attrs =
                            parse_replace_rmldoc_attr(&parsed_doc_file, &impl_item_const.attrs)?;
                    },
                    ImplItem::Fn(impl_item_fn) => {
                        impl_item_fn.attrs =
                            parse_replace_rmldoc_attr(&parsed_doc_file, &impl_item_fn.attrs)?;
                    },
                    ImplItem::Type(impl_item_type) => {
                        impl_item_type.attrs =
                            parse_replace_rmldoc_attr(&parsed_doc_file, &impl_item_type.attrs)?;
                    },
                    _ => {},
                }
            }

            let where_clause = &generics.where_clause;

            let mut trait_tokens = quote! {};

            if let Some((polarity, path, for_token)) = &trait_ {
                polarity.to_tokens(&mut trait_tokens);
                path.to_tokens(&mut trait_tokens);
                for_token.to_tokens(&mut trait_tokens);
            }

            let mut block = quote! {};

            brace_token.surround(&mut block, |block| {
                block.extend(quote! { #(#items)* });
            });

            // impls don't have doc comments on the actual impl item
            Ok(quote! {
                #(#attrs)*
                #defaultness #unsafety #impl_token #generics #trait_tokens #self_ty #where_clause #block
            })
        },
        Documentable::Type(item_type) => {
            if let Some((ref name, name_span)) = arguments.name {
                let ItemType {
                    attrs,
                    vis,
                    type_token,
                    ident,
                    generics,
                    eq_token,
                    ty,
                    semi_token,
                } = item_type;

                let refid = arguments.refid.as_ref().map(|(s, _)| s.as_str());
                let container_attrs =
                    make_doc_attributes(attrs.iter(), &parsed_doc_file, name, refid, name_span, false)?;

                Ok(quote! {
                    #(#container_attrs)*
                    #vis #type_token #ident #generics #eq_token #ty #semi_token
                })
            } else {
                Err(syn::Error::new(
                    arguments_span,
                    "`name` argument is required for documentation on `type` item",
                ))
            }
        },
        Documentable::Mod(item_mod) => {
            if let Some((ref name, name_span)) = arguments.name {
                let ItemMod {
                    attrs,
                    vis,
                    unsafety,
                    mod_token,
                    ident,
                    content,
                    semi,
                } = item_mod;

                if content.is_some() {
                    return Err(syn::Error::new(
                        arguments_span,
                        "module with block syntax is not supported yet",
                    ));
                }

                let refid = arguments.refid.as_ref().map(|(s, _)| s.as_str());
                let container_attrs =
                    make_doc_attributes(attrs.iter(), &parsed_doc_file, name, refid, name_span, false)?;

                Ok(quote! {
                    #(#container_attrs)*
                    #vis #mod_token #unsafety #ident #semi
                })
            } else {
                Err(syn::Error::new(
                    arguments_span,
                    "`name` argument is required for documentation on `type` item",
                ))
            }
        },
    }
}

#[cfg(not(feature = "generate-documentation"))]
fn strip_rmldoc_attributes(mut input: Documentable) -> syn::Result<TokenStream2> {
    Ok(match input {
        Documentable::Struct(ref mut item_struct) => {
            let ItemStruct { attrs, fields, .. } = item_struct;

            let filtered = remove_rmldoc_attrs(attrs)?;
            item_struct.attrs = filtered.cloned().collect::<Vec<_>>();

            match fields {
                Fields::Named(fields_named) => {
                    for named in &mut fields_named.named {
                        let new = remove_rmldoc_attrs(&named.attrs)?
                            .cloned()
                            .collect::<Vec<_>>();
                        named.attrs = new;
                    }
                },
                Fields::Unnamed(fields_unnamed) => {
                    for unnamed in &mut fields_unnamed.unnamed {
                        let new = remove_rmldoc_attrs(&unnamed.attrs)?
                            .cloned()
                            .collect::<Vec<_>>();
                        unnamed.attrs = new;
                    }
                },
                _ => {},
            }

            quote! {
                #item_struct
            }
        },
        Documentable::Enum(ref mut item_enum) => {
            let ItemEnum {
                attrs, variants, ..
            } = item_enum;

            let filtered = remove_rmldoc_attrs(attrs)?;
            item_enum.attrs = filtered.cloned().collect::<Vec<_>>();

            for variant in variants.iter_mut() {
                let new = remove_rmldoc_attrs(&variant.attrs)?
                    .cloned()
                    .collect::<Vec<_>>();
                variant.attrs = new;
            }

            quote! {
                #item_enum
            }
        },
        Documentable::Type(ref mut item_type) => {
            let ItemType { attrs, .. } = item_type;

            let filtered = remove_rmldoc_attrs(attrs)?;
            item_type.attrs = filtered.cloned().collect::<Vec<_>>();

            quote! { #item_type }
        },
        Documentable::Impl(ref mut item_impl) => {
            let ItemImpl { attrs, items, .. } = item_impl;

            let filtered = remove_rmldoc_attrs(attrs)?;
            item_impl.attrs = filtered.cloned().collect::<Vec<_>>();

            for item in items.iter_mut() {
                match item {
                    ImplItem::Const(impl_item_const) => {
                        let new = remove_rmldoc_attrs(&impl_item_const.attrs)?
                            .cloned()
                            .collect::<Vec<_>>();
                        impl_item_const.attrs = new;
                    },
                    ImplItem::Fn(impl_item_fn) => {
                        let new = remove_rmldoc_attrs(&impl_item_fn.attrs)?
                            .cloned()
                            .collect::<Vec<_>>();
                        impl_item_fn.attrs = new;
                    },
                    ImplItem::Type(impl_item_type) => {
                        let new = remove_rmldoc_attrs(&impl_item_type.attrs)?
                            .cloned()
                            .collect::<Vec<_>>();
                        impl_item_type.attrs = new;
                    },
                    _ => {},
                }
            }

            quote! {
                #item_impl
            }
        },
        Documentable::Trait(ref mut item_trait) => {
            let ItemTrait { attrs, items, .. } = item_trait;

            let filtered = remove_rmldoc_attrs(attrs)?;
            item_trait.attrs = filtered.cloned().collect::<Vec<_>>();

            for item in items.iter_mut() {
                match item {
                    TraitItem::Const(trait_item_const) => {
                        let new = remove_rmldoc_attrs(&trait_item_const.attrs)?
                            .cloned()
                            .collect::<Vec<_>>();
                        trait_item_const.attrs = new;
                    },
                    TraitItem::Fn(trait_item_fn) => {
                        let new = remove_rmldoc_attrs(&trait_item_fn.attrs)?
                            .cloned()
                            .collect::<Vec<_>>();
                        trait_item_fn.attrs = new;
                    },
                    TraitItem::Type(trait_item_type) => {
                        let new = remove_rmldoc_attrs(&trait_item_type.attrs)?
                            .cloned()
                            .collect::<Vec<_>>();
                        trait_item_type.attrs = new;
                    },
                    _ => {},
                }
            }

            quote! {
                #item_trait
            }
        },
        Documentable::Mod(ref mut item_mod) => {
            let ItemMod { attrs, .. } = item_mod;

            let filtered = remove_rmldoc_attrs(attrs)?;
            item_mod.attrs = filtered.cloned().collect::<Vec<_>>();

            quote! { #item_mod }
        },
    })
}

#[allow(unused)]
pub fn doc_gen(attrs: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let mut input = parse_macro_input!(item as Documentable);
    let input_span = input.span();

    let arguments = parse_macro_input!(attrs as ContainerDocArguments);

    #[cfg(feature = "generate-documentation")]
    let out = match make_doc_comments(&mut input, &arguments, input_span) {
        Ok(ts) => ts,
        Err(err) => return err.into_compile_error().into(),
    };
    #[cfg(not(feature = "generate-documentation"))]
    let out = match strip_rmldoc_attributes(input) {
        Ok(ts) => ts,
        Err(err) => return err.into_compile_error().into(),
    };

    out.into()
}
