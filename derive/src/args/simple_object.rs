use darling::{FromAttributes, ToTokens};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Generics, Path};

use crate::args::common;
use crate::args::common::{ArgImplementor, FieldImplementor};
use crate::utils::common::{
    CommonArg, CommonField, CommonInterfacable, CommonObject, GetArgs, GetFields,
};
use crate::utils::crate_name::get_crate_name;
use crate::utils::deprecation::Deprecation;
use crate::utils::derive_types::{BaseStruct, NamedField};
use crate::utils::error::IntoTokenStream;
use crate::utils::impl_block::BaseFnArg;
use crate::utils::interface_attr::InterfaceAttr;
use crate::utils::macros::*;
use crate::utils::rename_rule::RenameRule;
use crate::utils::with_attributes::WithAttributes;
use crate::utils::with_context::{MakeContext, WithContext};
use crate::utils::with_doc::WithDoc;

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct SimpleObjectFieldAttrs {
    #[darling(default)]
    pub skip: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub deprecation: Deprecation,
}

#[derive(Default, Debug, Clone)]
pub struct SimpleObjectFieldContext {
    pub rename_fields: Option<RenameRule>,
}

from_field!(
    SimpleObjectField,
    WithAttributes<
        WithDoc<SimpleObjectFieldAttrs>,
        WithContext<SimpleObjectFieldContext, NamedField>,
    >,
);

#[derive(FromAttributes, Debug, Clone)]
#[darling(attributes(graphql))]
pub struct SimpleObjectAttrs {
    #[darling(default)]
    pub root: bool,

    #[darling(skip)]
    pub mutation_root: bool,

    #[darling(default)]
    pub name: Option<String>,

    #[darling(default)]
    pub rename_fields: Option<RenameRule>,

    #[darling(default, multiple)]
    pub mark_as: Vec<InterfaceAttr>,

    #[darling(default, multiple)]
    pub mark_with: Vec<InterfaceAttr>,

    #[darling(default, multiple)]
    pub implement: Vec<InterfaceAttr>,
}

from_derive_input!(
    SimpleObject,
    WithAttributes<WithDoc<SimpleObjectAttrs>, BaseStruct<SimpleObjectField, Generics>>,
    ctx,
);

impl MakeContext<SimpleObjectFieldContext> for SimpleObject {
    fn make_context(&self) -> SimpleObjectFieldContext {
        SimpleObjectFieldContext {
            rename_fields: self.attrs.rename_fields,
        }
    }
}

impl CommonInterfacable for SimpleObject {
    fn get_mark_as(&self) -> &Vec<InterfaceAttr> {
        &self.attrs.mark_as
    }

    fn get_mark_with(&self) -> &Vec<InterfaceAttr> {
        &self.attrs.mark_with
    }

    fn get_implement(&self) -> &Vec<InterfaceAttr> {
        &self.attrs.implement
    }
}

impl CommonObject for SimpleObject {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_type(&self) -> darling::Result<Path> {
        Ok(self.ident.clone().into())
    }

    fn get_generics(&self) -> darling::Result<&Generics> {
        Ok(&self.generics)
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }
    fn get_fields_rename_rule(&self) -> Option<&RenameRule> {
        self.attrs.rename_fields.as_ref()
    }
}

impl CommonField for SimpleObjectField {
    fn get_name(&self) -> Option<&str> {
        self.attrs.name.as_deref()
    }

    fn get_ident(&self) -> darling::Result<&Ident> {
        Ok(&self.ident)
    }

    fn get_type(&self) -> darling::Result<&syn::Type> {
        Ok(&self.ty)
    }

    fn get_skip(&self) -> bool {
        self.attrs.skip
    }

    fn get_doc(&self) -> darling::Result<Option<String>> {
        Ok(self.attrs.doc.clone())
    }
    fn get_deprecation(&self) -> darling::Result<Deprecation> {
        Ok(self.attrs.deprecation.clone())
    }
    fn get_field_rename_rule(&self) -> Option<&RenameRule> {
        self.ctx.rename_fields.as_ref()
    }
}

impl FieldImplementor for SimpleObjectField {
    fn define_field(&self) -> darling::Result<TokenStream> {
        common::define_field(self)
    }

    fn get_execute_code(&self) -> darling::Result<TokenStream> {
        let resolver_ident = get_resolver_ident(self)?;

        Ok(quote! {
            let parent = ctx.parent_value.try_downcast_ref::<Self>()?;
            let value = Self::#resolver_ident(parent);
        })
    }

    fn get_resolve_code(&self) -> darling::Result<TokenStream> {
        let crate_name = get_crate_name();
        Ok(quote! {
            #crate_name::ResolveRef::resolve_ref(value, &ctx)
        })
    }

    fn get_field_argument_definition(&self) -> darling::Result<TokenStream> {
        Ok(quote!())
    }

    fn get_field_description_code(&self) -> darling::Result<TokenStream> {
        common::field_description(self)
    }

    fn get_field_deprecation_code(&self) -> darling::Result<TokenStream> {
        common::field_deprecation_code(self)
    }

    fn get_field_usage_code(&self) -> darling::Result<TokenStream> {
        Ok(quote! {
            let object = object.field(field);
        })
    }
}

impl GetFields<SimpleObjectField> for SimpleObject {
    fn get_fields(&self) -> darling::Result<&Vec<SimpleObjectField>> {
        Ok(&self.data.fields)
    }
}

struct SimpleObjectArg;

impl CommonArg for SimpleObjectArg {
    fn get_name(&self) -> Option<&str> {
        unreachable!("SimpleObjectArg has no name")
    }

    fn get_index(&self) -> usize {
        0
    }

    fn get_arg(&self) -> &BaseFnArg {
        unreachable!("SimpleObjectArg has no arg")
    }

    fn is_marked_as_ctx(&self) -> bool {
        false
    }
}

impl ArgImplementor for SimpleObjectArg {
    fn get_self_arg_definition(&self) -> darling::Result<TokenStream> {
        unreachable!("SimpleObjectArg has no definition")
    }

    fn get_typed_arg_definition(&self) -> darling::Result<TokenStream> {
        unreachable!("SimpleObjectArg has no definition")
    }

    fn get_self_arg_usage(&self) -> darling::Result<TokenStream> {
        common::get_self_arg_usage(self)
    }

    fn get_typed_arg_usage(&self) -> darling::Result<TokenStream> {
        unreachable!("SimpleObjectArg has no usage")
    }
}

static EMPTY_ARGS: Vec<SimpleObjectArg> = Vec::new();

impl GetArgs<SimpleObjectArg> for SimpleObjectField {
    fn get_args(&self) -> darling::Result<&Vec<SimpleObjectArg>> {
        Ok(&EMPTY_ARGS)
    }
}

fn get_resolver_ident(field: &impl CommonField) -> darling::Result<Ident> {
    let field_ident = field.get_ident()?;
    let resolver_name = format!("__resolve_{}", field_ident);

    let resolver_ident = syn::Ident::new(&resolver_name, field_ident.span());
    Ok(resolver_ident)
}

fn impl_resolver(field: &impl CommonField) -> darling::Result<TokenStream> {
    let field_ident = field.get_ident()?;
    let resolver_ident = get_resolver_ident(field)?;
    let ty = field.get_type()?;
    Ok(quote! {
        fn #resolver_ident(&self) -> &#ty {
            &self.#field_ident
        }
    })
}

fn impl_resolvers<O, F>(object: &O) -> darling::Result<TokenStream>
where
    O: CommonObject + GetFields<F>,
    F: CommonField,
{
    let ident = object.get_ident();
    let fields = object
        .get_fields()?
        .iter()
        .filter(|field| !field.get_skip())
        .map(impl_resolver)
        .map(|r| r.into_token_stream())
        .collect::<Vec<TokenStream>>();
    let (impl_generics, ty_generics, where_clause) = object.get_generics()?.split_for_impl();
    Ok(quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #(#fields)*
        }
    })
}

fn root_register_code(object: &SimpleObject) -> TokenStream {
    let root = if object.attrs.root {
        let crate_name = get_crate_name();
        Some(quote! {
            let registry = registry.set_root(<Self as #crate_name::Object>::NAME);
        })
    } else {
        None
    };
    let mutation_root = if object.attrs.mutation_root {
        let crate_name = get_crate_name();
        Some(quote! {
            let registry = registry.set_mutation(<Self as #crate_name::Object>::NAME);
        })
    } else {
        None
    };
    quote!(#root #mutation_root)
}

fn impl_register(object: &SimpleObject) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();

    let root_register = root_register_code(object);

    let ident = &object.ident;
    let define_object = common::impl_define_object();
    let add_interfaces = common::get_interface_code(object)?;
    let implement = common::get_add_implement_code(object, &object.attrs.implement)?;

    let description = common::object_description(object.get_doc()?.as_deref())?;
    let define_fields = common::get_define_fields_code(object)?;
    let register_object_code = common::register_object_code();

    let (impl_generics, ty_generics, where_clause) = object.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics #crate_name::Register for #ident #ty_generics #where_clause {
            fn register(registry: #crate_name::Registry) -> #crate_name::Registry {

                #root_register

                #define_object

                #implement

                #add_interfaces

                #description

                #define_fields

                #register_object_code
            }
        }
    })
}

impl ToTokens for SimpleObject {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let impl_object = common::impl_object(self).into_token_stream();
        let impl_resolve_owned = common::impl_resolve_owned(self).into_token_stream();
        let impl_resolve_ref = common::impl_resolve_ref(self).into_token_stream();
        let impl_resolvers = impl_resolvers(self).into_token_stream();
        let impl_register = impl_register(self).into_token_stream();
        let impl_interface_mark = common::impl_interface_mark(self).into_token_stream();
        tokens.extend(quote! {
            #impl_object
            #impl_interface_mark
            #impl_resolve_owned
            #impl_resolve_ref
            #impl_resolvers
            #impl_register
        })
    }
}
