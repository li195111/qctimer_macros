extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

use syn::ItemFn;

#[proc_macro_attribute]
pub fn timer(args: TokenStream, input: TokenStream) -> TokenStream {
    timer_impl(args, input, false)
}

#[proc_macro_attribute]
pub fn async_timer(args: TokenStream, input: TokenStream) -> TokenStream {
    timer_impl(args, input, true)
}

fn timer_impl(_args: TokenStream, input: TokenStream, is_async: bool) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let name = &input.sig.ident; // 獲取函數名稱
    let name_str = name.to_string();
    let vis = &input.vis; // 獲取函數的可見性
    let block = &input.block; // 獲取函數的實現
    let inputs = &input.sig.inputs; // 獲取函數的參數
    let output = &input.sig.output; // 獲取函數的返回值

    let is_ansyc_block = if is_async {
        quote! { async }
    } else {
        quote! {}
    };

    let async_timer_block = if is_async {
        quote! { let res = async move { #block }.await; }
    } else {
        quote! { let res = (|| { #block })(); }
    };

    // 構造新的函數
    let expanded = quote! {
        #vis #is_ansyc_block fn #name(#inputs) #output {
            use chrono::{Duration, prelude::Utc};

            fn friendly_duration(duration: Duration) -> String {
                let nanos = duration.num_nanoseconds().unwrap_or(0);
                match nanos {
                    0..=999 => format!("{} ns.", nanos),
                    1_000..=999_999 => format!("{} µs.", nanos / 1_000),
                    1_000_000..=999_999_999 => format!("{} ms.", nanos / 1_000_000),
                    _ => format!("{} s.", nanos / 1_000_000_000),
                }
            }

            let st = Utc::now();
            #async_timer_block
            let ed = Utc::now();

            let duration = ed - st;
            println!("\n{} Time Cost: {}",  #name_str, friendly_duration(duration));

            res
        }
    };

    TokenStream::from(expanded)
}
