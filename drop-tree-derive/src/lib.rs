use deluxe::ParseMetaItem;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::spanned::Spanned;
use syn::{
    Field, Fields, FieldsNamed, GenericParam, Ident, ImplGenerics, ItemStruct, Lifetime,
    LifetimeParam, Path, TypeGenerics, TypeParamBound, WhereClause, parse_quote,
    parse_quote_spanned,
};

#[derive(ParseMetaItem, Debug)]
struct DropTreeAttributes {
    // `Some` if the struct borrows ownership from another struct
    borrows: Option<Path>,

    // TODO: make ownership thread safe - not implemented yet
    // #[deluxe(default)]
    // unsync: bool,

    // `Some` if a destructor function was passed in
    destructor: Option<Ident>,
}

fn replace_last_ident(mut path: Path, new_ident: Ident) -> Path {
    if let Some(last) = path.segments.last_mut() {
        last.ident = new_ident;
    }
    path
}

struct DestructorExpansion {
    data_struct_drop_impl: TokenStream,
    data_struct_body: TokenStream,
    data_struct_impl_generics: TokenStream,
    user_struct_body: TokenStream,
    data_struct_deref_impls: TokenStream,
}

impl DestructorExpansion {
    fn new(
        user_struct_ident: &Ident,
        data_struct_ident: &Ident,
        dt_erased_ownership_handle_ty: &Ident,
        user_fields: &[Field],
        destructor_fn: Option<&Ident>,
        user_impl_generics: &ImplGenerics,
        user_ty_generics: &TypeGenerics,
        user_where_clause: &Option<&WhereClause>,
    ) -> Self {
        let drop_body = if let Some(destructor_fn) = destructor_fn {
            quote! {
                #destructor_fn(self);
            }
        } else {
            quote! {}
        };

        Self {
            // always implement Drop on the data struct
            data_struct_drop_impl: quote! {
                #[automatically_derived]
                impl #user_impl_generics Drop for #data_struct_ident #user_ty_generics #user_where_clause {
                    fn drop(&mut self) {
                        #drop_body
                    }
                }
            },

            // user fields always live in the data struct
            data_struct_body: quote! {
                {
                    #(#user_fields),*
                }
            },

            // data struct always carries generics
            data_struct_impl_generics: quote! {
                #user_impl_generics
            },

            // always provide Deref / DerefMut
            data_struct_deref_impls: quote! {
                #[automatically_derived]
                impl #user_impl_generics ::std::ops::Deref
                    for #user_struct_ident #user_ty_generics #user_where_clause
                {
                    type Target = #data_struct_ident #user_ty_generics;

                    fn deref(&self) -> &Self::Target {
                        // SAFETY:
                        // - `_links` owns the erased data struct
                        unsafe { self._links.get_self().unerased() }
                    }
                }

                #[automatically_derived]
                impl #user_impl_generics ::std::ops::DerefMut
                    for #user_struct_ident #user_ty_generics #user_where_clause
                {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        // SAFETY:
                        // - `_links` owns the erased data struct
                        // - exclusive access via &mut self
                        unsafe { self._links.get_mut_self().unerased_mut() }
                    }
                }
            },

            // outer struct is always just a handle
            user_struct_body: quote! {
                _links: #dt_erased_ownership_handle_ty,
                _phantom: ::std::marker::PhantomData<
                    #data_struct_ident #user_ty_generics
                >
            },
        }
    }
}

struct BorrowExpansion {
    ownership_link_type_alias_value: TokenStream,
    constructor_impl: TokenStream,
}

impl BorrowExpansion {
    fn root(
        ownership_token_ty: &proc_macro2::TokenStream,
        marker_struct_ident: &Ident,
        data_struct_ident: &Ident,
        user_struct_ident: &Ident,
        user_struct_fields_no_vis: &[Field],
        user_struct_field_idents: &[Ident],
        impl_generics: &ImplGenerics,
        ty_generics: &TypeGenerics,
        where_clause: &Option<&WhereClause>,
    ) -> Self {
        Self {
            // the type definition of self marker with empty parent
            ownership_link_type_alias_value: quote! {
                ::drop_tree::OwnershipLink<#ownership_token_ty<#marker_struct_ident>, ()>
            },
            // no `parent` needs to be passed as an argument to constructor
            constructor_impl: {
                quote! {
                    impl #impl_generics #user_struct_ident #ty_generics #where_clause {
                        pub(crate) fn new_with_borrow(
                            #(#user_struct_fields_no_vis),*
                        ) -> Self {
                            Self {
                                _links: ::drop_tree::OwnershipLink::new_root(#data_struct_ident {
                                    #(#user_struct_field_idents),*
                                }),
                                _phantom: ::std::marker::PhantomData,
                            }
                        }
                    }
                }
            },
        }
    }

    fn child(
        dt_erased_ownership_handle_ty: &proc_macro2::TokenStream,
        dt_ownership_borrow_trair: &proc_macro2::TokenStream,
        marker_struct_ident: &Ident,
        data_struct_ident: &Ident,
        parent_struct_path: &Path,
        user_struct_ident: &Ident,
        user_struct_fields_no_vis: &[Field],
        user_struct_field_idents: &[Ident],
        impl_generics: &ImplGenerics,
        ty_generics: &TypeGenerics,
        where_clause: &Option<&WhereClause>,
    ) -> Self {
        let last_segment_ident = &parent_struct_path.segments.last().unwrap().ident;

        let new_ident = Ident::new(
            &format!("{}Marker", last_segment_ident),
            last_segment_ident.span(),
        );

        let parent_marker_path = replace_last_ident(parent_struct_path.clone(), new_ident);

        Self {
            // same as above but parent is populated with the borrowd structs token
            ownership_link_type_alias_value: quote! {
                ::drop_tree::OwnershipLink<
                    #dt_erased_ownership_handle_ty<#marker_struct_ident>,
                    #dt_erased_ownership_handle_ty<#parent_marker_path>
                >
            },
            // a node that borrows from the parent must take the parent as an argument
            // if the node also has a destructor, the fields are slightly different
            constructor_impl: {
                quote! {
                    impl #impl_generics #user_struct_ident #ty_generics #where_clause {
                        pub(crate) fn new_with_borrow(
                            #(#user_struct_fields_no_vis,)*
                            parent: &impl #dt_ownership_borrow_trair<#parent_marker_path>
                        ) -> Self {
                            Self {
                                _links: parent.ownership_borrow::<Self, #data_struct_ident>(#data_struct_ident {
                                    #(#user_struct_field_idents),*
                                }),
                                _phantom: ::std::marker::PhantomData,
                            }
                        }
                    }
                }
            },
        }
    }
}

/// Enables hierarchical ownership for a struct.
///
/// Applying `#[drop_tree]` makes the annotated type participate in an
/// ownership tree, meaning that:
/// - parent nodes always outlive their children
/// - destructors run in a bottom-up order
///
/// The macro generates the internal ownership code required to:
/// - safely borrow from a parent node when constructing a child
/// - optionally run a custom destructor with access to the node’s fields.
///
/// # Arguments
/// - `borrow(path_to_parent)` — takes in an explicit full path to a parent struct that
///   this child node will borrow from. When the child node is constructed, it will make
///   sure the parent cannot be destroyed until the child destroyed first.
/// - `destructor(function_name)` — registers a function to be called when the node
///   is dropped. The function must take a `DropCtx<Self>` parameter, which contains
///   mutable references to each field in the original struct.
///
/// # Notes
/// This macro generates additional hidden types and trait implementations.
/// Users should construct nodes via the generated `new_with_borrow` functions.
#[proc_macro_attribute]
pub fn drop_tree(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let user_struct = syn::parse_macro_input!(input as ItemStruct);

    let drop_tree_attrs = match deluxe::parse::<DropTreeAttributes>(args) {
        Ok(attrs) => attrs,
        Err(err) => return err.to_compile_error().into(),
    };

    let ItemStruct {
        attrs: user_struct_attrs,
        vis: user_struct_vis,
        struct_token: user_struct_token,
        ident: user_struct_ident,
        generics: user_struct_generics,
        fields: user_struct_fields,
        ..
    } = user_struct;

    // name of the sealed module used to contain all items
    let sealed_node_mod_ident =
        Ident::new(&format!("{}Sealed", user_struct_ident), Span::call_site());

    // a marker struct stored inside the erased `OwnershipHandle` to make each erased type unique enough to prevent unsoundness
    let marker_struct_ident =
        Ident::new(&format!("{}Marker", user_struct_ident), Span::call_site());

    // if destructor isnt provided, this is a ZST.
    // if a destructor is provided, this contains all the user's fields, and the user struct just becomes
    // essentially a transparent wrapper around this struct.
    // this is because the `Drop` impl needs to be implemented on this struct and it's likey a user with
    // an explicit destructor wants to access fields during drop, so the fields need to be on this marker
    let data_struct_ident = Ident::new(&format!("{}Data", user_struct_ident), Span::call_site());

    // type alias that wraps `drop_tree::OwnershipLink<SelfMarker, ParentMarker>`
    // mainly to reduce duplication
    let ownership_link_type_alias = Ident::new(
        &format!("{}OwnershipLink", user_struct_ident),
        Span::call_site(),
    );

    let Fields::Named(FieldsNamed {
        brace_token,
        named: user_fields,
    }) = user_struct_fields
    else {
        return syn::Error::new_spanned(user_struct_ident, "only named structs are supported")
            .to_compile_error()
            .into();
    };

    // TODO: async versions
    let (dt_erased_ownership_handle_ty, dt_ownership_borrow_trait) = (
        quote!(::drop_tree::sync::ErasedOwnershipHandle),
        quote!(::drop_tree::sync::OwnershipBorrow),
    );

    let (user_impl_generics, user_ty_generics, user_where_clause) =
        user_struct_generics.split_for_impl();

    // extracts names of struct fields, ignoring types
    let user_field_idents: Vec<Ident> = user_fields
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();

    // recreates the struct fields but making them `pub(super)` as the struct is part of a module
    let pub_super_user_fields: Vec<Field> = user_fields
        .iter()
        .map(|f| {
            let Field {
                attrs,
                ident,
                colon_token,
                ty,
                ..
            } = f;

            parse_quote_spanned!(ty.span() => #(#attrs)* pub(super) #ident #colon_token #ty)
        })
        .collect();

    // some generics need to be bound by `'static`
    let mut user_generics_static_bound = user_struct_generics.clone();

    user_generics_static_bound
        .params
        .iter_mut()
        .for_each(|param| match param {
            GenericParam::Type(type_param) => type_param
                .bounds
                .push(TypeParamBound::Lifetime(parse_quote!('static))),
            _ => {},
        });

    let (user_impl_generics_static_bound, ..) = user_generics_static_bound.split_for_impl();

    let DestructorExpansion {
        data_struct_drop_impl,
        data_struct_body,
        data_struct_impl_generics,
        user_struct_body,
        data_struct_deref_impls,
    } = DestructorExpansion::new(
            &user_struct_ident,
            &data_struct_ident,
            &ownership_link_type_alias,
            &pub_super_user_fields,
            drop_tree_attrs.destructor.as_ref(),
            &user_impl_generics_static_bound,
            &user_ty_generics,
            &user_where_clause,
        );

    // recreates the struct fields but removing the vis token for use as arguments
    let user_fields_no_vis: Vec<Field> = user_fields
        .iter()
        .map(|f| {
            let Field {
                attrs,
                ident,
                colon_token,
                ty,
                ..
            } = f;

            parse_quote_spanned!(ty.span() => #(#attrs)* #ident #colon_token #ty)
        })
        .collect();

    let BorrowExpansion {
        ownership_link_type_alias_value,
        constructor_impl,
    } = match &drop_tree_attrs.borrows {
        Some(parent_node) => BorrowExpansion::child(
            &dt_erased_ownership_handle_ty,
            &dt_ownership_borrow_trait,
            &marker_struct_ident,
            &data_struct_ident,
            parent_node,
            &user_struct_ident,
            &user_fields_no_vis,
            &user_field_idents,
            &user_impl_generics_static_bound,
            &user_ty_generics,
            &user_where_clause,
        ),
        None => BorrowExpansion::root(
            &dt_erased_ownership_handle_ty,
            &marker_struct_ident,
            &data_struct_ident,
            &user_struct_ident,
            &user_fields_no_vis,
            &user_field_idents,
            &user_impl_generics_static_bound,
            &user_ty_generics,
            &user_where_clause,
        ),
    };

    // implements `drop_tree::OwnershipBorrow`
    // used for a child to borrow from a parent
    // the marker struct is important here as it prevents an erased handle from the wrong parent
    // being passed in, as the two markers wouldn't match
    // if that was allowed, the casts inside `ErasedOwnershipHandle::erased`
    // and `ErasedOwnershipHandle::erased_mut` would be UB
    let dt_ownership_borrow_impl = quote! {
        #[automatically_derived]
        impl #user_impl_generics #dt_ownership_borrow_trait<#marker_struct_ident> for #user_struct_ident #user_ty_generics #user_where_clause {
            fn ownership_borrow<Child: 'static, ChildData>(
                &self,
                child: ChildData,
            ) -> ::drop_tree::OwnershipLink<
               #dt_erased_ownership_handle_ty<Child::SelfToken>,
               #dt_erased_ownership_handle_ty<Self::SelfToken>,
            >
            where
                Child: ::drop_tree::OwnershipNode,
                ChildData: ::drop_tree::OwnershipNodeData<Child::SelfToken>
            {
                self._links.borrow_with_child(child)
            }
        }
    };

    // we extract out the fields to modify them, but we want to put them back into
    // the same braces for span reasons
    let mut user_struct_body_braced = TokenStream::new();
    brace_token.surround(&mut user_struct_body_braced, |tokens| {
        tokens.extend(quote!(#user_struct_body));
    });

    // the extra lifetime required the associated type `drop_tree::DestructorContext::Ctx`
    let drop_ctx_lifetime = Lifetime::new("'__ctx", Span::call_site());
    let drop_ctx_lifetime_param = LifetimeParam::new(drop_ctx_lifetime.clone());

    // due to the extra lifetime required by the associated type on `drop_tree::DestructorContext`,
    // we need to modify the original generics to include that extra lifetime, and also bound generics
    // by that lifetime
    let mut user_struct_generics_added_lifetimes = user_generics_static_bound.clone();

    // adds the lifetime as a generic type param
    user_struct_generics_added_lifetimes
        .params
        .push(GenericParam::from(drop_ctx_lifetime_param));

    // adds the lifetime as a bound to any existing generic types
    user_struct_generics_added_lifetimes
        .params
        .iter_mut()
        .for_each(|param| match param {
            GenericParam::Type(type_param) => type_param
                .bounds
                .push(TypeParamBound::Lifetime(drop_ctx_lifetime.clone())),
            _ => {},
        });

    let (user_struct_generics_for_drop_ctx_impl, ..) =
        user_struct_generics_added_lifetimes.split_for_impl();

    quote! {
        #[allow(non_snake_case)]
        #[doc(hidden)]
        mod #sealed_node_mod_ident {
            use super::*;

            #[doc(hidden)]
            pub struct #marker_struct_ident(());

            #[doc(hidden)]
            pub struct #data_struct_ident #data_struct_impl_generics #data_struct_body

            #data_struct_drop_impl
            
            #[doc(hidden)]
            pub type #ownership_link_type_alias = #ownership_link_type_alias_value;
            
            #(#user_struct_attrs)*
            pub #user_struct_token #user_struct_ident #user_impl_generics_static_bound #user_struct_body_braced

            // prevents user from implementing `Drop` themselves
            #[automatically_derived]
            impl #user_impl_generics Drop for #user_struct_ident #user_ty_generics #user_where_clause {
                fn drop(&mut self) {}
            }
            
            #data_struct_deref_impls
            #constructor_impl
            #dt_ownership_borrow_impl

            #[automatically_derived]
            impl #user_impl_generics ::drop_tree::OwnershipNode for #user_struct_ident #user_ty_generics #user_where_clause {
                type SelfToken = #marker_struct_ident;
            }

            #[automatically_derived]
            impl #user_struct_generics_for_drop_ctx_impl ::drop_tree::DestructorContext<#drop_ctx_lifetime> for #user_struct_ident #user_ty_generics #user_where_clause {
                type Ctx = #data_struct_ident #user_ty_generics;
            }
        }

        #[doc(hidden)]
        pub(crate) use #sealed_node_mod_ident::{#marker_struct_ident};
        #user_struct_vis use #sealed_node_mod_ident::#user_struct_ident;
    }
    .into()
}
