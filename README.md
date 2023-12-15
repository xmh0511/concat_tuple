````rust
use concat_tuple::{concat_tuple, Combine, Product, ProductList, Tuple};

fn main() {
    type TT = <(u32, char) as Tuple>::ProductList;
    let t: Product<u32, Product<char, ()>> = TT::default();
    type T3 = <TT as Combine<Product<String, ()>>>::Output;
	let t: (u32, char, String) = <<T3 as ProductList>::Tuple>::default();
    let t: Product<u32, Product<char, Product<String, ()>>> = T3::default();
    let t: (u32, char, String) = t.as_tuple();
    let t: Product<u32, Product<char, Product<String, ()>>> = t.as_product();
    let t: (i32, char, String, i32, u8, f64) = concat_tuple((1, 'c'), (String::new(),1,2u8,3f64));
}

````