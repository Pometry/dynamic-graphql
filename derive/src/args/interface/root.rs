use proc_macro2::TokenStream;
use quote::quote;

use crate::args::common::{
    get_field_name, get_field_type, get_type_name, ArgImplementor, FieldImplementor,
};
use crate::args::interface::{InterfaceMethod, InterfaceMethodArg};
use crate::args::{common, Interface};
use crate::utils::common::{CommonObject, GetArgs};
use crate::utils::crate_name::get_crate_name;
use crate::utils::interface_hash::get_interface_hash;

impl FieldImplementor for InterfaceMethod {
    fn define_field(&self) -> darling::Result<TokenStream> {
        let crate_name = get_crate_name();
        let field_name = get_field_name(self)?;
        let field_type = get_field_type(self)?;
        Ok(quote! {
            let field = #crate_name::dynamic::InterfaceField::new(
                #field_name,
                <#field_type as #crate_name::GetOutputTypeRef>::get_output_type_ref(),
            );
        })
    }

    fn get_execute_code(&self) -> darling::Result<TokenStream> {
        unreachable!("Interface method can't be executed")
    }

    fn get_resolve_code(&self) -> darling::Result<TokenStream> {
        unreachable!("Interface method can't be resolved")
    }

    fn get_field_argument_definition(&self) -> darling::Result<TokenStream> {
        common::get_argument_definitions(self.get_args()?)
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

impl ArgImplementor for InterfaceMethodArg {
    fn get_self_arg_definition(&self) -> darling::Result<TokenStream> {
        let arg_ident = common::get_arg_ident(self);

        Ok(quote! {
            let parent = ctx.parent_value.try_downcast_ref::<I>()?;
            let #arg_ident = parent;
        })
    }

    fn get_typed_arg_definition(&self) -> darling::Result<TokenStream> {
        common::get_typed_arg_definition(self)
    }

    fn get_self_arg_usage(&self) -> darling::Result<TokenStream> {
        common::get_self_arg_usage(self)
    }

    fn get_typed_arg_usage(&self) -> darling::Result<TokenStream> {
        common::get_typed_arg_usage(self)
    }
}

pub fn define_interface_struct(input: &Interface) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let name = get_type_name(input)?;
    let hash = get_interface_hash(&name);

    let ident = &input.arg.ident;
    Ok(quote! {
        pub struct #ident<'__dynamic_graphql_lifetime, T=#crate_name::InterfaceRoot>(::std::marker::PhantomData<T>, #crate_name::AnyBox<'__dynamic_graphql_lifetime>);
        impl #crate_name::GraphqlType for #ident<'static> {
            const NAME: &'static str = #name;
        }
        impl #crate_name::OutputType for #ident<'static> {}
        impl #crate_name::Interface for #ident<'static> {
            const MARK: u64 = #hash;
        }
    })
}

pub fn impl_register(input: &Interface) -> darling::Result<TokenStream> {
    let crate_name = get_crate_name();
    let ident = &input.arg.ident;

    let description = common::object_description(input.get_doc()?.as_deref())?;
    let define_fields = common::get_define_fields_code(input)?;
    let register_code = common::register_object_code();
    Ok(quote! {
        impl #crate_name::Register for #ident <'static> {
            fn register(registry: #crate_name::Registry) -> #crate_name::Registry {
                // todo rename to interface
                let object = #crate_name::dynamic::Interface::new(<Self as #crate_name::Interface>::NAME);

                #description
                #define_fields
                #register_code
            }
        }
    })
}
