extern crate proc_macro;
extern crate syn;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn run_puzzle(tokens: TokenStream) -> TokenStream {
    let input = tokens.to_string();
    let args: Vec<syn::Ident> = input
        .split(',')
        .map(|arg| syn::parse_str(arg).unwrap())
        .collect();
    let get_puzzle_fn = &args[0];
    let day_lit = &args[1];
    let part_lit = &args[2];

    let path_names = |day: &str| {
        let path = format!("advent_of_code_2023::day{day}");
        (
            syn::parse_str(&format!("{}::part1", path)).unwrap(),
            syn::parse_str(&format!("{}::part2", path)).unwrap(),
        )
    };

    let modules: Vec<_> = (1..=5)
        .map(|day| {
            let day_str = day.to_string();
            let day = day_str.as_str();
            let (part1, part2): (syn::Path, syn::Path) = path_names(day);

            quote! {
                #day => {
                    match #part_lit.as_str() {
                        "1" => { #part1(&input) },
                        "2" => { #part2(&input) },
                        _ => { panic!("Invalid part") }
                    }
                },
            }
        })
        .collect();

    let expanded = quote! {
        {
            let input = #get_puzzle_fn(day);
            match #day_lit.as_str() {
                #(#modules)*
                _ => panic!("Day not found"),
            }
        }
    };

    expanded.into()
}
