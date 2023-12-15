use std::marker::PhantomData;

use concat_tuple::{concat_tuple, Combine, Product, ProductList, Tuple, MergeProductList};

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

struct Conjunction<T>(T);

impl<T> Conjunction<T>{
	fn and<Y>(self,v:Y)-> Conjunction<<T as MergeProductList<Product<Y,()>>>::Output>
	where T:MergeProductList<Product<Y,()>>
	{
		Conjunction(self.0.concat(Product(v,())))
	}
	fn to_tuple(self)-><T as ProductList>::Tuple where T:ProductList{
		todo!()
	}
}

fn show(){
	let c = Conjunction(());
	let c = c.and(1);
	let c = c.and(1u8);
	let t = c.to_tuple();
}
