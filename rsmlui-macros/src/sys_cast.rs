use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, TokenStreamExt, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    Attribute, Data, DeriveInput, Expr, Fields, Ident, Meta, MetaNameValue, Path, Result, Token,
    Type, Visibility, token,
};

use crate::utils::ErrorSpan;

bitflags::bitflags! {
    #[derive(PartialEq, Copy, Clone)]
    struct GenFlags: u8 {
        const Ref = 1;
        const Slice = 1 << 1;
    }
}

mod keywords {
    syn::custom_keyword!(pair);
    syn::custom_keyword!(from);
    syn::custom_keyword!(transparent);
    syn::custom_keyword!(fields);
}

#[allow(clippy::large_enum_variant)]
enum CastTypeArgs {
    // `struct(...)`
    Struct(StructArgs),
    // `bitflags(...)`
    Bitflag(BitflagArgs),
    // `enum(...)`
    Enum(EnumArgs),
}

struct FromPair {
    sys_type: Path,
    rust_type: Type,
}

struct SysCastArgs {
    // `gen_ref`, `gen_slice`, etc
    flags: GenFlags,
    // `struct(...)`, `enum(...)` argument
    type_args: CastTypeArgs,
}

enum FromSpec {
    Single(Path),
    Generic(Vec<FromPair>),
}

enum TransparentArgs {
    Plain,
    Fields { fields: Vec<Ident> },
}

impl Parse for TransparentArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: keywords::transparent = input.parse()?;

        if !input.peek(token::Paren) {
            return Ok(Self::Plain);
        }

        let transparent_content;
        syn::parenthesized!(transparent_content in input);

        let _: keywords::fields = transparent_content.parse()?;

        let fields_content;
        syn::parenthesized!(fields_content in transparent_content);

        let mut fields = vec![];

        loop {
            if fields_content.is_empty() {
                break;
            }

            let ident: Ident = fields_content.parse()?;

            fields.push(ident);

            if fields_content.is_empty() {
                break;
            }

            fields_content.parse::<Token![,]>()?;
        }

        Ok(Self::Fields { fields })
    }
}

struct StructArgs {
    from: FromSpec,
    transparent: Option<TransparentArgs>,
}

impl Parse for FromPair {
    fn parse(input: ParseStream) -> Result<Self> {
        let pair_kw: keywords::pair = input.parse()?;

        let content;
        syn::parenthesized!(content in input);

        let mut sys_type = None;
        let mut rust_type = None;

        loop {
            if content.is_empty() {
                break;
            }

            if content.peek(keywords::from) {
                if sys_type.is_some() {
                    return Err(content.error("`from` specified more than once"));
                }

                content.parse::<keywords::from>()?;
                content.parse::<Token![=]>()?;
                sys_type = Some(content.parse::<Path>()?);
            } else if content.peek(Token![self]) {
                if rust_type.is_some() {
                    return Err(content.error("`self` specified more than once"));
                }

                content.parse::<Token![self]>()?;
                content.parse::<Token![=]>()?;
                rust_type = Some(content.parse::<syn::Type>()?);
            } else {
                return Err(content.error("expected `from = ...` or `self = ...`"));
            }

            if content.is_empty() {
                break;
            }

            content.parse::<Token![,]>()?;
        }

        let sys_type =
            sys_type.ok_or_else(|| pair_kw.error("`from` is required in `pair(...)`"))?;
        let rust_type =
            rust_type.ok_or_else(|| pair_kw.error("`self` is required in `pair(...)`"))?;

        Ok(Self {
            sys_type,
            rust_type,
        })
    }
}

struct BitflagArgs {
    from: Path,
    repr: Path,
}

struct EnumArgs {
    from: Path,
    repr: Path,
}

struct SysCoreMapping {
    sys: Ident,
    rust: Ident,
}

impl Parse for SysCastArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut flags = GenFlags::empty();
        let mut type_args: Option<CastTypeArgs> = None;

        loop {
            if input.is_empty() {
                break;
            }

            if input.peek(Token![struct]) {
                let kw: Token![struct] = input.parse()?;

                let content;
                syn::parenthesized!(content in input);

                if type_args.is_some() {
                    return Err(kw.error("only one type argument is allowed"));
                }

                type_args = Some(CastTypeArgs::Struct(content.parse()?));
            } else if input.peek(Token![enum]) {
                let kw: Token![enum] = input.parse()?;

                let content;
                syn::parenthesized!(content in input);

                if type_args.is_some() {
                    return Err(kw.error("only one type argument is allowed"));
                }

                type_args = Some(CastTypeArgs::Enum(content.parse()?));
            } else {
                let meta: Meta = input.parse()?;

                if meta.path().is_ident("bitflags") {
                    let list = meta.require_list()?;

                    if type_args.is_some() {
                        return Err(list.error("only one type argument is allowed"));
                    }

                    type_args = Some(CastTypeArgs::Bitflag(syn::parse2(list.tokens.clone())?));
                } else if meta.path().is_ident("gen_ref") {
                    meta.require_path_only()?;

                    flags |= GenFlags::Ref;
                } else if meta.path().is_ident("gen_slice") {
                    meta.require_path_only()?;

                    flags |= GenFlags::Slice;
                } else {
                    return Err(meta.error("unexpected argument"));
                }
            }

            if input.is_empty() {
                break;
            }

            let _: Token![,] = input.parse()?;
        }

        if type_args.is_none() {
            return Err(
                input.error("expected one of `struct(...)`, `enum(...)`, or `bitflags(...)`")
            );
        }

        Ok(Self {
            flags,
            type_args: type_args.unwrap(),
        })
    }
}

impl Parse for StructArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut from = None;
        let mut transparent = None;

        loop {
            if input.is_empty() {
                break;
            }

            if input.peek(keywords::from) {
                if from.is_some() {
                    return Err(input.error("`from` argument must only exist once"));
                }

                let from_kw: keywords::from = input.parse()?;

                if input.peek(Token![=]) {
                    input.parse::<Token![=]>()?;

                    from = Some(FromSpec::Single(input.parse()?))
                } else if input.peek(token::Paren) {
                    let content;
                    syn::parenthesized!(content in input);

                    let pairs = Punctuated::<FromPair, Token![,]>::parse_terminated(&content)?;

                    if pairs.is_empty() {
                        return Err(from_kw.error("`from(...)` requires at least one `pair(...)`"));
                    }

                    from = Some(FromSpec::Generic(pairs.into_iter().collect()))
                } else {
                    return Err(from_kw.error("expected `= Path` or `(pair(...)...)`"));
                };
            } else if input.peek(keywords::transparent) {
                if transparent.is_some() {
                    return Err(input.error("`transparent` argument must only exist once"));
                }

                transparent = Some(input.parse()?);
            } else {
                return Err(input.error("expected `from` or `transparent`"));
            }

            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }

        if from.is_none() {
            return Err(input.error("`from` is a required argument"));
        }

        Ok(Self {
            from: from.unwrap(),
            transparent,
        })
    }
}

impl Parse for EnumArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let arguments: Punctuated<MetaNameValue, Token![,]> = Punctuated::parse_terminated(input)?;
        let mut from = None;
        let mut repr = None;

        for argument in arguments.iter() {
            if argument.path.is_ident("from") {
                if from.is_some() {
                    return Err(argument.error("`from` argument must only exist once"));
                }
                match &argument.value {
                    Expr::Path(path) => from = Some(path.path.clone()),
                    _ => return Err(argument.error("`from` argument must be a path")),
                }

                continue;
            }
            if argument.path.is_ident("repr") {
                if repr.is_some() {
                    return Err(argument.error("`repr` argument must only exist once"));
                }

                match &argument.value {
                    Expr::Path(path) => repr = Some(path.path.clone()),
                    _ => return Err(argument.error("`repr` argument must be a type")),
                }

                continue;
            }

            return Err(argument.error("unexpected argument"));
        }

        if from.is_none() {
            return Err(input.error("`from` argument is required"));
        }
        if repr.is_none() {
            return Err(input.error("`repr` argument is required"));
        }

        Ok(EnumArgs {
            from: from.unwrap(),
            repr: repr.unwrap(),
        })
    }
}

impl Parse for BitflagArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        let arguments: Punctuated<MetaNameValue, Token![,]> = Punctuated::parse_terminated(input)?;

        let mut from = None;
        let mut repr = None;

        for argument in arguments.iter() {
            if argument.path.is_ident("from") {
                if from.is_some() {
                    return Err(argument.error("`from` argument must only exist once"));
                }

                match &argument.value {
                    Expr::Path(path) => from = Some(path.path.clone()),
                    _ => return Err(argument.error("`from` argument must be a path")),
                }

                continue;
            }

            if argument.path.is_ident("repr") {
                if repr.is_some() {
                    return Err(argument.error("`repr` argument must only exist once"));
                }

                match &argument.value {
                    Expr::Path(path) => repr = Some(path.path.clone()),
                    _ => return Err(argument.error("`repr` argument must be a type")),
                }

                continue;
            }

            return Err(argument.error("unexpected argument"));
        }

        if from.is_none() {
            return Err(input.error("`from` argument is required"));
        }

        if repr.is_none() {
            return Err(input.error("`repr` argument is required"));
        }

        Ok(Self {
            from: from.unwrap(),
            repr: repr.unwrap(),
        })
    }
}

fn extract_sys_name(attrs: &[Attribute]) -> Result<Option<Ident>> {
    for attr in attrs {
        if attr.path().is_ident("sys") {
            return Ok(Some(attr.parse_args::<Ident>()?));
        }
    }
    Ok(None)
}

fn remove_sys_attrs(attrs: &mut Vec<Attribute>) {
    attrs.retain(|a| !a.path().is_ident("sys"));
}

fn generate_owned_from_sys<S: ToTokens, N: ToTokens>(sys_type: &S, item_name: &N) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl crate::FromSys<#sys_type> for #item_name {
            #[inline(always)]
            fn from_sys(value: #sys_type) -> Self {
                ::core::assert!(
                    unsafe { <Self as crate::utils::conversions::SysCast>::validate(&raw const value) },
                    concat!("sys_cast invariant violated: invalid sys value for `", stringify!(#item_name), "`"),
                );
                unsafe { ::std::mem::transmute::<#sys_type, Self>(value) }
            }
        }
    }
}

fn generate_ref_from_sys<S: ToTokens, N: ToTokens>(sys_type: &S, item_name: &N) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl<'__a> crate::FromSys<&'__a #sys_type> for &'__a #item_name {
            #[inline(always)]
            fn from_sys(value: &'__a #sys_type) -> Self {
                unsafe { &*(value as *const #sys_type as *const #item_name) }
            }
        }
    }
}

fn generate_slice_from_sys<S: ToTokens, N: ToTokens>(sys_type: &S, item_name: &N) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl<'__a> crate::FromSys<&'__a [#sys_type]> for &'__a [#item_name] {
            #[inline(always)]
            fn from_sys(value: &'__a [#sys_type]) -> Self {
                unsafe {
                    ::std::slice::from_raw_parts(value.as_ptr() as *const #item_name, value.len())
                }
            }
        }
    }
}

fn generate_owned_into_sys<S: ToTokens, N: ToTokens>(sys_type: &S, item_name: &N) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl crate::IntoSys<#sys_type> for #item_name {
            #[inline(always)]
            fn into_sys(self) -> #sys_type {
                unsafe { ::std::mem::transmute::<Self, #sys_type>(self) }
            }
        }
    }
}

fn generate_ref_into_sys<S: ToTokens, N: ToTokens>(sys_type: &S, item_name: &N) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl<'__a> crate::IntoSys<&'__a #sys_type> for &'__a #item_name {
            #[inline(always)]
            fn into_sys(self) -> &'__a #sys_type {
                unsafe { &*(self as *const #item_name as *const #sys_type) }
            }
        }
    }
}

fn generate_slice_into_sys<S: ToTokens, N: ToTokens>(sys_type: &S, item_name: &N) -> TokenStream2 {
    quote! {
        #[automatically_derived]
        impl<'__a> crate::IntoSys<&'__a [#sys_type]> for &'__a [#item_name] {
            #[inline(always)]
            fn into_sys(self) -> &'__a [#sys_type] {
                unsafe {
                    ::std::slice::from_raw_parts(self.as_ptr() as *const #sys_type, self.len())
                }
            }
        }
    }
}

fn generate_extra_impls<S: ToTokens, N: ToTokens>(
    flags: GenFlags,
    sys_type: &S,
    item_name: &N,
) -> TokenStream2 {
    let mut out = quote! {};

    if flags.contains(GenFlags::Ref) {
        let from = generate_ref_from_sys(sys_type, item_name);
        let into = generate_ref_into_sys(sys_type, item_name);

        out = quote! { #out #from #into };
    }

    if flags.contains(GenFlags::Slice) {
        let from = generate_slice_from_sys(sys_type, item_name);
        let into = generate_slice_into_sys(sys_type, item_name);

        out = quote! { #out #from #into };
    }

    out
}

fn substitute_type(ty: &Type, subst: &[(Ident, Type)]) -> Type {
    match ty {
        Type::Path(tp) if tp.qself.is_none() => {
            if let Some(ident) = tp.path.get_ident()
                && let Some((_, concrete)) = subst.iter().find(|(p, _)| p == ident)
            {
                return concrete.clone();
            }
            let mut tp = tp.clone();
            for seg in &mut tp.path.segments {
                if let syn::PathArguments::AngleBracketed(ab) = &mut seg.arguments {
                    for arg in &mut ab.args {
                        if let syn::GenericArgument::Type(t) = arg {
                            *t = substitute_type(t, subst);
                        }
                    }
                }
            }
            Type::Path(tp)
        },
        Type::Reference(r) => {
            let mut r = r.clone();
            r.elem = Box::new(substitute_type(&r.elem, subst));
            Type::Reference(r)
        },
        Type::Slice(s) => {
            let mut s = s.clone();
            s.elem = Box::new(substitute_type(&s.elem, subst));
            Type::Slice(s)
        },
        Type::Array(a) => {
            let mut a = a.clone();
            a.elem = Box::new(substitute_type(&a.elem, subst));
            Type::Array(a)
        },
        other => other.clone(),
    }
}

fn generate_struct(
    args: &StructArgs,
    cast_args: &SysCastArgs,
    mut input: DeriveInput,
) -> Result<TokenStream2> {
    fn generate_struct_offset_assertion<N: ToTokens, P: ToTokens>(
        struct_name: &N,
        sys_type: &P,
        sys_name: &Ident,
        rust_name: &Ident,
    ) -> TokenStream2 {
        quote! {
            ::core::assert!(
                ::std::mem::offset_of!(#struct_name, #rust_name)
                    == ::std::mem::offset_of!(#sys_type, #sys_name),
                concat!(
                    "offset mismatch: `",
                    stringify!(#struct_name), "::", stringify!(#rust_name),
                    "` vs `",
                    stringify!(#sys_type), "::", stringify!(#sys_name),
                    "`"
                )
            );
        }
    }

    fn generate_struct_size_alignment_assertions<T: ToTokens, S: ToTokens>(
        rust_type: &T,
        sys_type: &S,
    ) -> TokenStream2 {
        quote! {
            ::core::assert!(
                ::std::mem::size_of::<#rust_type>()
                    == ::std::mem::size_of::<#sys_type>(),
                concat!(
                    "size mismatch: `",
                    stringify!(#rust_type),
                    "` vs `",
                    stringify!(#sys_type), "`"
                )
            );
            ::core::assert!(
                ::std::mem::align_of::<#rust_type>()
                    == ::std::mem::align_of::<#sys_type>(),
                concat!(
                    "alignment mismatch: `",
                    stringify!(#rust_type),
                    "` vs `",
                    stringify!(#sys_type),
                    "`"
                )
            );
        }
    }

    fn generate_struct_validate_fields<T: ToTokens, P: ToTokens>(
        field_ty: &T,
        sys_type: &P,
        sys_name: &Ident,
    ) -> TokenStream2 {
        quote! {
            <#field_ty as crate::utils::conversions::SysCast>::validate(
                (ptr as *const u8).add(::std::mem::offset_of!(#sys_type, #sys_name))
                    as *const <#field_ty as crate::utils::conversions::SysCast>::Sys
            )
        }
    }

    if args.transparent.is_some() {
        input.attrs.push(syn::parse_quote!(#[repr(transparent)]));
    } else {
        input.attrs.push(syn::parse_quote!(#[repr(C)]));
    }

    let inner_type: Option<Type> = if args.transparent.is_some() {
        match &input.data {
            Data::Struct(s) => match &s.fields {
                Fields::Unnamed(u) if u.unnamed.len() == 1 => Some(u.unnamed[0].ty.clone()),
                Fields::Unnamed(_) => {
                    return Err(input
                        .ident
                        .error("transparent struct must have exactly one field"));
                },
                _ => {
                    return Err(input
                        .ident
                        .error("transparent struct must be a tuple struct"));
                },
            },
            _ => return Err(input.ident.error("expected a struct")),
        }
    } else {
        None
    };

    let fields = match &mut input.data {
        Data::Struct(s) => &mut s.fields,
        _ => return Err(input.ident.error("expected a struct")),
    };

    let mut field_mappings: Vec<(Ident, Ident, syn::Type)> = vec![];

    match fields {
        Fields::Named(named) => {
            for field in named.named.iter_mut() {
                let rust_name = field.ident.as_ref().unwrap().clone();
                let sys_name = extract_sys_name(&field.attrs)?.unwrap_or_else(|| rust_name.clone());
                remove_sys_attrs(&mut field.attrs);
                field_mappings.push((rust_name, sys_name, field.ty.clone()));
            }
        },
        Fields::Unnamed(_) if args.transparent.is_some() => {},
        Fields::Unnamed(_) => {
            return Err(input
                .ident
                .error("`sys_cast` does not support tuple structs"));
        },
        Fields::Unit => {},
    }

    match &args.from {
        FromSpec::Single(sys_type) => {
            let struct_name = &input.ident;

            if let Some(transparent_args) = &args.transparent {
                let inner_type = inner_type.as_ref().unwrap();
                let size_alignment =
                    generate_struct_size_alignment_assertions(struct_name, sys_type);

                let offset_assertions: Vec<TokenStream2> = match transparent_args {
                    TransparentArgs::Plain => vec![],
                    TransparentArgs::Fields { fields } => fields
                        .iter()
                        .map(|field| {
                            quote! {
                                ::core::assert!(
                                    ::std::mem::offset_of!(#inner_type, #field)
                                        == ::std::mem::offset_of!(#sys_type, #field),
                                    concat!(
                                        "offset mismatch: `",
                                        stringify!(#inner_type), "::", stringify!(#field),
                                        "` vs `",
                                        stringify!(#sys_type), "::", stringify!(#field),
                                        "`"
                                    )
                                );
                            }
                        })
                        .collect(),
                };

                let owned_from = generate_owned_from_sys(sys_type, struct_name);
                let owned_into = generate_owned_into_sys(sys_type, struct_name);
                let extra = generate_extra_impls(cast_args.flags, sys_type, struct_name);

                return Ok(quote! {
                    #input

                    const _: () = {
                        #size_alignment
                        #(#offset_assertions)*
                    };

                    #[automatically_derived]
                    unsafe impl crate::utils::conversions::SysCast for #struct_name {
                        type Sys = #sys_type;
                    }

                    #owned_from
                    #extra
                    #owned_into
                });
            }

            let mut offset_assertions = vec![];
            let mut validate_fields = vec![];
            let mut field_ty_checks = vec![];

            for (rust_name, sys_name, field_ty) in &field_mappings {
                offset_assertions.push(generate_struct_offset_assertion(
                    struct_name,
                    sys_type,
                    sys_name,
                    rust_name,
                ));

                field_ty_checks.push(quote! { _field_check::<#field_ty>(); });
                validate_fields.push(generate_struct_validate_fields(
                    field_ty, sys_type, sys_name,
                ));
            }

            let owned_from = generate_owned_from_sys(sys_type, struct_name);
            let owned_into = generate_owned_into_sys(sys_type, struct_name);
            let extra = generate_extra_impls(cast_args.flags, sys_type, struct_name);
            let size_alignment = generate_struct_size_alignment_assertions(struct_name, sys_type);

            Ok(quote! {
                #input

                const _: () = {
                    #size_alignment
                    #(#offset_assertions)*

                    #[allow(unused)]
                    const fn _field_check<T: crate::utils::conversions::SysCast>() {}
                    #(#field_ty_checks)*
                };

                #[automatically_derived]
                unsafe impl crate::utils::conversions::SysCast for #struct_name {
                    type Sys = #sys_type;

                    unsafe fn validate(ptr: *const #sys_type) -> bool {
                        unsafe { true #(&& #validate_fields)* }
                    }
                }

                #owned_from
                #extra
                #owned_into
            })
        },

        FromSpec::Generic(pairs) => {
            let type_params: Vec<Ident> = input
                .generics
                .params
                .iter()
                .filter_map(|p| match p {
                    syn::GenericParam::Type(tp) => Some(tp.ident.clone()),
                    _ => None,
                })
                .collect();

            let pair_impls = pairs
                .iter()
                .map(|pair| {
                    let sys_type = &pair.sys_type;
                    let rust_type = &pair.rust_type;

                    let concrete_args: Vec<Type> = match rust_type {
                        Type::Path(tp) => tp
                            .path
                            .segments
                            .last()
                            .and_then(|s| match &s.arguments {
                                syn::PathArguments::AngleBracketed(ab) => Some(
                                    ab.args
                                        .iter()
                                        .filter_map(|a| match a {
                                            syn::GenericArgument::Type(t) => Some(t.clone()),
                                            _ => None,
                                        })
                                        .collect(),
                                ),
                                _ => None,
                            })
                            .unwrap_or_default(),
                        _ => vec![],
                    };

                    // Zip to build substitution map: V → Vec2, W → IVec2, etc.
                    let subst: Vec<(Ident, Type)> = type_params
                        .iter()
                        .zip(concrete_args.iter())
                        .map(|(p, t)| (p.clone(), t.clone()))
                        .collect();

                    let mut offset_assertions = vec![];
                    let mut validate_fields = vec![];
                    let mut field_ty_checks = vec![];

                    for (rust_name, sys_name, field_ty) in &field_mappings {
                        let concrete_field_ty = substitute_type(field_ty, &subst);

                        offset_assertions.push(generate_struct_offset_assertion(
                            rust_type, sys_type, sys_name, rust_name,
                        ));
                        field_ty_checks.push(quote! { _field_check::<#concrete_field_ty>(); });
                        validate_fields.push(generate_struct_validate_fields(
                            &concrete_field_ty,
                            sys_type,
                            sys_name,
                        ));
                    }

                    let owned_from = generate_owned_from_sys(sys_type, rust_type);
                    let owned_into = generate_owned_into_sys(sys_type, rust_type);
                    let extra = generate_extra_impls(cast_args.flags, sys_type, rust_type);
                    let size_alignment =
                        generate_struct_size_alignment_assertions(rust_type, sys_type);

                    quote! {
                        const _: () = {
                            #size_alignment
                            #(#offset_assertions)*

                            #[allow(unused)]
                            const fn _field_check<T: crate::utils::conversions::SysCast>() {}
                            #(#field_ty_checks)*
                        };

                        #[automatically_derived]
                        unsafe impl crate::utils::conversions::SysCast for #rust_type {
                            type Sys = #sys_type;

                            unsafe fn validate(ptr: *const #sys_type) -> bool {
                                unsafe { true #(&& #validate_fields)* }
                            }
                        }

                        #owned_from
                        #extra
                        #owned_into
                    }
                })
                .collect::<Vec<_>>();

            Ok(quote! {
                #input

                #(#pair_impls)*
            })
        },
    }
}

fn generate_enum(
    args: &EnumArgs,
    cast_args: &SysCastArgs,
    mut input: DeriveInput,
) -> Result<TokenStream2> {
    let sys_type = &args.from;
    let repr = &args.repr;

    input.attrs.push(syn::parse_quote!(#[repr(#repr)]));

    let variants = match &mut input.data {
        Data::Enum(e) => &mut e.variants,
        _ => return Err(input.ident.error("expected an enum")),
    };

    let mut mappings: Vec<SysCoreMapping> = vec![];

    for variant in variants.iter_mut() {
        let rust = variant.ident.clone();

        let sys = extract_sys_name(&variant.attrs)?.unwrap_or_else(|| rust.clone());

        remove_sys_attrs(&mut variant.attrs);

        mappings.push(SysCoreMapping { rust, sys });
    }

    let enum_name = &input.ident;

    let discriminant_assertions = mappings
        .iter()
        .map(|m| {
            let rust = &m.rust;
            let sys = &m.sys;
            quote! {
                ::core::assert!(
                    #enum_name::#rust as #repr == #sys_type::#sys as #repr,
                    concat!(
                        "discriminant mismatch: `",
                        stringify!(#enum_name), "::", stringify!(#rust),
                        "` vs `",
                        stringify!(#sys_type), "::", stringify!(#sys),
                        "`"
                    )
                );
            }
        })
        .collect::<Vec<_>>();

    let is_valid_body = if mappings.is_empty() {
        quote! { false }
    } else {
        let checks = mappings
            .iter()
            .map(|m| {
                let rust = &m.rust;

                quote! { (raw == #enum_name::#rust as #repr) }
            })
            .collect::<Vec<_>>();

        quote! { #(#checks)||* }
    };

    let owned_from = generate_owned_from_sys(&sys_type, &enum_name);
    let owned_into = generate_owned_into_sys(&sys_type, &enum_name);
    let extra = generate_extra_impls(cast_args.flags, &sys_type, &enum_name);

    Ok(quote! {
        #input

        const _: () = {
            ::core::assert!(
                ::std::mem::size_of::<#enum_name>() == ::std::mem::size_of::<#sys_type>(),
                concat!("size mismatch: `", stringify!(#enum_name), "` vs `", stringify!(#sys_type), "`")
            );
            ::core::assert!(
                ::std::mem::align_of::<#enum_name>() == ::std::mem::align_of::<#sys_type>(),
                concat!("alignment mismatch: `", stringify!(#enum_name), "` vs `", stringify!(#sys_type), "`")
            );

            #(#discriminant_assertions)*
        };

        #[automatically_derived]
        unsafe impl crate::utils::conversions::SysCast for #enum_name {
            type Sys = #sys_type;

            unsafe fn validate(ptr: *const #sys_type) -> bool {
                let raw: #repr = unsafe { *(ptr as *const #repr) };
                #is_valid_body
            }
        }

        #owned_from
        #extra
        #owned_into
    })
}

struct BitflagsContentConst {
    pub attrs: Vec<Attribute>,
    pub const_: Token![const],
    pub ident: Ident,
    pub equals: Token![=],
    pub expr: Expr,
    pub semi: Token![;],
}

impl Parse for BitflagsContentConst {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let const_ = input.parse()?;
        let ident = input.parse()?;
        let equals = input.parse()?;
        let expr = input.parse()?;
        let semi = input.parse()?;

        Ok(Self {
            attrs,
            const_,
            ident,
            equals,
            expr,
            semi,
        })
    }
}

impl ToTokens for BitflagsContentConst {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            attrs,
            const_,
            ident,
            equals,
            expr,
            semi,
        } = self;

        tokens.append_all(quote! {
            #(#attrs)*
            #const_ #ident #equals #expr #semi
        });
    }
}

struct BitflagsContent {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub struct_: Token![struct],
    pub ident: Ident,
    pub colon: Token![:],
    pub repr: Ident,
    pub brace_token: token::Brace,
    pub content: Vec<BitflagsContentConst>,
}

impl Parse for BitflagsContent {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse()?;
        let struct_ = input.parse()?;
        let ident = input.parse()?;
        let colon = input.parse()?;
        let repr = input.parse()?;

        let content;
        let brace_token = syn::braced!(content in input);

        let mut consts = vec![];
        while !content.is_empty() {
            consts.push(content.parse()?);
        }

        Ok(Self {
            attrs,
            vis,
            struct_,
            ident,
            colon,
            repr,
            brace_token,
            content: consts,
        })
    }
}

impl ToTokens for BitflagsContent {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            attrs,
            vis,
            struct_,
            ident,
            colon,
            repr,
            brace_token,
            content,
        } = self;

        let mut braced_content = TokenStream2::new();
        brace_token.surround(&mut braced_content, |tokens| {
            tokens.append_all(quote! {
                #(#content)*
            });
        });

        tokens.append_all(quote! {
            #(#attrs)*
            #vis #struct_ #ident #colon #repr #braced_content
        });
    }
}

struct BitflagsInvocation {
    pub path: Path,
    pub bang_token: Token![!],
    pub brace_token: token::Brace,
    pub content: BitflagsContent,
}

impl ToTokens for BitflagsInvocation {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            path,
            bang_token,
            brace_token,
            content,
        } = self;

        let mut braced_content = TokenStream2::new();
        brace_token.surround(&mut braced_content, |tokens| {
            tokens.append_all(quote! {
                #content
            });
        });

        tokens.append_all(quote! {
            #path #bang_token #braced_content
        });
    }
}

impl Parse for BitflagsInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let path = input.parse()?;
        let bang_token = input.parse()?;

        let content;
        let brace_token = syn::braced!(content in input);
        let content = content.parse()?;

        Ok(Self {
            path,
            bang_token,
            brace_token,
            content,
        })
    }
}

fn generate_bitflags(
    args: &BitflagArgs,
    cast_args: &SysCastArgs,
    input: TokenStream2,
) -> Result<TokenStream2> {
    let mut macro_invocation: BitflagsInvocation = syn::parse2(input)?;

    let sys_type = &args.from;
    let repr = &args.repr;

    if !repr.is_ident(&macro_invocation.content.repr) {
        return Err(repr.error("sys_cast repr does not match bitflags repr"));
    }

    let flags_name = &macro_invocation.content.ident;

    let mut mappings: Vec<SysCoreMapping> = vec![];

    for const_def in macro_invocation.content.content.iter_mut() {
        let rust = const_def.ident.clone();

        let sys = extract_sys_name(&const_def.attrs)?.unwrap_or_else(|| rust.clone());

        remove_sys_attrs(&mut const_def.attrs);

        mappings.push(SysCoreMapping { rust, sys });
    }

    let flag_assertions = mappings
        .iter()
        .map(|m| {
            let sys = &m.sys;
            let rust = &m.rust;

            quote! {
                ::core::assert!(
                    #flags_name::#rust.bits() == #sys_type::#sys.0,
                    concat!(
                        "flag value mismatch: `",
                        stringify!(#flags_name), "::", stringify!(#rust),
                        "` vs `",
                        stringify!(#sys_type), "::", stringify!(#sys),
                        "`"
                    )
                );
            }
        })
        .collect::<Vec<_>>();

    let owned_from = generate_owned_from_sys(&sys_type, &flags_name);
    let owned_into = generate_owned_into_sys(&sys_type, &flags_name);
    let extra = generate_extra_impls(cast_args.flags, &sys_type, &flags_name);

    remove_sys_attrs(&mut macro_invocation.content.attrs);

    Ok(quote! {
        #macro_invocation

        const _: () = {
            ::core::assert!(
                ::std::mem::size_of::<#flags_name>() == ::std::mem::size_of::<#sys_type>(),
                concat!("size mismatch: `", stringify!(#flags_name), "` vs `", stringify!(#sys_type), "`")
            );
            ::core::assert!(
                ::std::mem::align_of::<#flags_name>() == ::std::mem::align_of::<#sys_type>(),
                concat!("alignment mismatch: `", stringify!(#flags_name), "` vs `", stringify!(#sys_type), "`")
            );
            #(#flag_assertions)*
        };

        #[automatically_derived]
        unsafe impl crate::utils::conversions::SysCast for #flags_name {
            type Sys = #sys_type;

            unsafe fn validate(ptr: *const #sys_type) -> bool {
                let raw: #repr = unsafe { *(ptr as *const #repr) };
                raw & !<Self as ::bitflags::Flags>::all().bits() == 0
            }
        }

        #owned_from
        #extra
        #owned_into
    })
}

pub fn generate(attrs: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let sys_cast_args = match syn::parse::<SysCastArgs>(attrs) {
        Ok(a) => a,
        Err(e) => return e.into_compile_error().into(),
    };

    let result = match &sys_cast_args.type_args {
        CastTypeArgs::Struct(args) => match syn::parse::<DeriveInput>(item) {
            Ok(input) => generate_struct(args, &sys_cast_args, input),
            Err(e) => Err(e),
        },
        CastTypeArgs::Enum(args) => match syn::parse::<DeriveInput>(item) {
            Ok(input) => generate_enum(args, &sys_cast_args, input),
            Err(e) => Err(e),
        },
        CastTypeArgs::Bitflag(args) => generate_bitflags(args, &sys_cast_args, item.into()),
    };

    match result {
        Ok(ts) => ts.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
