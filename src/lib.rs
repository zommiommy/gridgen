use proc_macro::{TokenStream, TokenTree};
use quote::{format_ident, quote};
// hic sunt leones, this code is not for the faint of heart and its has to be
// rewritten using the crate nom.

#[proc_macro]
pub fn grid_gen_bench(input: TokenStream) -> TokenStream {
    gen_grid(input, |name, func, args, generics| {
        quote!(
            #[bench]
            #[automatically_derived]
            pub fn #name(b: &mut Bencher) {
                #func::<#generics>(b, #args);
            }
        )
    })
}

#[proc_macro]
pub fn grid_gen_test(input: TokenStream) -> TokenStream {
    gen_grid(input, |name, func, args, generics| {
        quote!(
            #[test]
            #[automatically_derived]
            pub fn #name() {
                #func::<#generics>(#args);
            }
        )
    })
}

fn gen_grid<F>(input: TokenStream, func: F) -> TokenStream
where
    F: Fn(
        proc_macro2::TokenStream,
        proc_macro2::Ident,
        proc_macro2::TokenStream,
        proc_macro2::TokenStream,
    ) -> proc_macro2::TokenStream,
{
    let mut input = input.into_iter().peekable();
    let test_func = format_ident!("{}", input.next().unwrap().to_string());
    assert_eq!(input.next().unwrap().to_string(), ",");

    let test_name_prefix = input.next().unwrap().to_string();
    assert_eq!(input.next().unwrap().to_string(), ",");

    let mut args = Vec::new();
    let mut generics = Vec::new();
    while input.peek().is_some() {
        let mut is_generic = false;
        let mut arg_name = input.next().unwrap().to_string();
        if arg_name == "generic" {
            arg_name = input.next().unwrap().to_string();
            is_generic = true;
        }
        assert_eq!(input.next().unwrap().to_string(), ":");
        let arg_value_tokens = input.next().unwrap();
        let mut arg_values = Vec::new();
        match arg_value_tokens {
            TokenTree::Group(group) => {
                assert!(group.delimiter() == proc_macro::Delimiter::Bracket);
                let mut args_iter = group.stream().into_iter().peekable();
                while args_iter.peek().is_some() {
                    let arg_value = args_iter.next().unwrap().to_string();
                    if args_iter.peek().is_none() {
                        arg_values.push((arg_value.clone(), arg_value));
                        continue;
                    }

                    let next = args_iter.next().unwrap().to_string();
                    match next.as_str() {
                        "," => {
                            arg_values.push((arg_value.clone(), arg_value));
                        }
                        "=" => {
                            assert_eq!(args_iter.next().unwrap().to_string(), ">");
                            let arg_name = args_iter
                                .next()
                                .unwrap()
                                .to_string()
                                .trim_matches('"')
                                .to_string();
                            arg_values.push((arg_value, arg_name));
                            if args_iter.peek().is_some() {
                                assert_eq!(args_iter.next().unwrap().to_string(), ",");
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => unreachable!(),
        }
        if is_generic {
            generics.push((arg_name, arg_values));
        } else {
            args.push((arg_name, arg_values));
        }
        if input.peek().is_some() {
            assert_eq!(input.next().unwrap().to_string(), ",");
        }
    }

    let mut res = TokenStream::new();
    let mut indices = vec![0; generics.len() + args.len()];
    loop {
        let mut test_name = vec![test_name_prefix.clone()];
        let mut test_args = vec![];
        let mut test_generics = vec![];

        for (i, (_name, values)) in generics.iter().enumerate() {
            let (value, name) = &values[indices[i]];
            test_name.push(name.clone());
            test_generics.push(value.clone());
        }

        for (i, (_name, values)) in args.iter().enumerate() {
            let (value, name) = &values[indices[generics.len() + i]];
            test_name.push(name.clone());
            test_args.push(value.clone());
        }

        let test_name: proc_macro2::TokenStream = test_name.join("_").parse().unwrap();
        let test_args: proc_macro2::TokenStream = test_args.join(",").parse().unwrap();
        let test_generics: proc_macro2::TokenStream = test_generics.join(",").parse().unwrap();

        let to_append: TokenStream =
            func(test_name, test_func.clone(), test_args, test_generics).into();
        res.extend(to_append);

        for i in (0..indices.len()).rev() {
            if i < generics.len() {
                if indices[i] < generics[i].1.len() - 1 {
                    indices[i] += 1;
                    break;
                } else {
                    indices[i] = 0;
                }
            } else if indices[i] < args[i - generics.len()].1.len() - 1 {
                indices[i] += 1;
                break;
            } else {
                indices[i] = 0;
            }
        }

        if indices.iter().all(|&x| x == 0) {
            break;
        }
    }

    res
}
