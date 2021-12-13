extern crate proc_macro;

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Grammar)]
pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = input.ident;
    let struct_val_name = struct_name.to_string();

    let output = quote!{
        impl Grammar for #struct_name {
            fn process(&mut self, token: &token::Token) -> Result { return self.pattern.execute(token); }
            fn is_done(&self) -> bool { return self.pattern.is_done; }
            fn info(&self) -> String { return format!("{}:[{}]", #struct_val_name, self.pattern.state); }
        }
    };

    return proc_macro::TokenStream::from(output);
}

#[cfg(test)]
mod tests {
    #[test]
    fn name() {
        unimplemented!();
    }
}