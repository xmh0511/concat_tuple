#[derive(Debug, Default)]
pub struct Product<T, U>(pub T, pub U);

pub trait Tuple {
    type ProductList: ProductList;
    fn as_product(self) -> Self::ProductList;
}

pub trait ProductList {
    type Tuple: Tuple;
    fn as_tuple(self) -> Self::Tuple;
}

pub trait MergeProductList<U: ProductList> {
    type Output;
    fn concat(self, v: U) -> Self::Output;
}

pub trait Combine<U> {
    type Output;
}

impl<U> Combine<U> for () {
    type Output = U;
}

impl<T1, T2, U> Combine<U> for Product<T1, T2>
where
    T2: Combine<U>,
{
    type Output = Product<T1, T2::Output>;
}

macro_rules! gen_product_list {
	($id:ident)=>{
		Product<$id,()>
	};
	($id:ident, $($tail:ident),*) => {
		Product<$id,gen_product_list!($($tail),*)>
	};
	($id:ident,($v:ident))=>{
		Product<$id,$v>
	};
	($id:ident, $($tail:ident),*,($v:ident)) => {
		Product<$id,gen_product_list!($($tail),*,($v))>
	};
}

macro_rules! create_product_pattern {
	($id:ident)=>{
		Product($id,())
	};
	($id:ident, $($tail:ident),*) => {
		Product($id,create_product_pattern!($($tail),*))
	};
	($id:ident,($v:ident))=>{
		Product($id,$v)
	};
	($id:ident,$($tail:ident),*,($v:ident))=>{
		Product($id,create_product_pattern!($($tail),*,($v)))
	}
}

macro_rules! gen_impl {
	($($ids:ident)+) => {
		impl<$($ids),+> Tuple for ($($ids),+,){
			type ProductList = gen_product_list!($($ids),+);
			#[allow(non_snake_case)]
			fn as_product(self)->Self::ProductList{
				let ($($ids),+,) = self;
				create_product_pattern!($($ids),+)
			}
		}
		impl<$($ids),+> ProductList for gen_product_list!($($ids),+){
			type Tuple = ($($ids),+,);
			#[allow(non_snake_case)]
			fn as_tuple(self)->Self::Tuple{
				let create_product_pattern!($($ids),+) = self;
				($($ids),+,)
			}

		}
		impl<U:ProductList,$($ids),+> MergeProductList<U> for gen_product_list!($($ids),+){
			type Output = gen_product_list!($($ids),+,(U));
			#[allow(non_snake_case)]
			fn concat(self,v:U)-> Self::Output{
				let create_product_pattern!($($ids),+) = self;
				create_product_pattern!($($ids),+,(v))
			}
		}
	};
}

pub type Tuple2ProductList<T> = <T as Tuple>::ProductList;
pub type ProductList2Tuple<T> = <T as ProductList>::Tuple;
pub type ConCatTuple<T, U> = <Tuple2ProductList<T> as Combine<Tuple2ProductList<U>>>::Output;

pub fn concat_tuple<T: Tuple, U: Tuple>(
    t1: T,
    t2: U,
) -> <<Tuple2ProductList<T> as MergeProductList<Tuple2ProductList<U>>>::Output as ProductList>::Tuple
where
    Tuple2ProductList<T>: MergeProductList<Tuple2ProductList<U>>,
    <Tuple2ProductList<T> as MergeProductList<Tuple2ProductList<U>>>::Output: ProductList,
{
    t1.as_product().concat(t2.as_product()).as_tuple()
}

gen_impl! {T0}
gen_impl! {T0 T1}
gen_impl! {T0 T1 T2}
gen_impl! {T0 T1 T2 T3}
gen_impl! {T0 T1 T2 T3 T4}
gen_impl! {T0 T1 T2 T3 T4 T5}
gen_impl! {T0 T1 T2 T3 T4 T5 T6}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29 T30}
gen_impl! {T0 T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16 T17 T18 T19 T20 T21 T22 T23 T24 T25 T26 T27 T28 T29 T30 T31}
