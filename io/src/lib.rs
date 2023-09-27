use proc_macro::TokenStream;
use quote::quote;

// Create a procedural attribute macro
//
// Notably, this must be placed alone in its own crate
#[proc_macro_attribute]
pub fn print_caller_location(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the passed item as a function
    let func = syn::parse_macro_input!(item as syn::ItemFn);

    // Break the function down into its parts
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = func;

    // Extract function name for prettier output
    let name = format!("{}", sig.ident);

    // Generate the output, adding `#[track_caller]` as well as a `println!`
    let output = quote! {
        #[track_caller]
        #(#attrs)*
        #vis #sig {
            println!(
                "entering `fn {}`: called from `{}`",
                #name,
                ::core::panic::Location::caller()
            );
            #block
        }
    };

    // Convert the output from a `proc_macro2::TokenStream` to a `proc_macro::TokenStream`
    TokenStream::from(output)
}
