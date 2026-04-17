use proc_macro2::TokenStream;
use quote::quote;
fn get_impl() -> TokenStream {
    #[cfg(feature = "units")]
    quote! {
        impl<const N:usize>
    }
    #[cfg(not(feature = "units"))]
    quote! {
        impl
    }
}
fn get_impl_generic() -> TokenStream {
    #[cfg(feature = "units")]
    quote! {
        impl<T,const N:usize>
    }
    #[cfg(not(feature = "units"))]
    quote! {
        impl<T>
    }
}
fn get_type(token: TokenStream) -> TokenStream {
    #[cfg(feature = "units")]
    quote! {
        Number<#token, f32, N>
    }
    #[cfg(not(feature = "units"))]
    quote! {
        Number<#token>
    }
}
#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}
impl Op {
    fn get(self) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
        match self {
            Op::Add => (
                quote! {Add},
                quote! {AddAssign},
                quote! {add},
                quote! {add_assign},
            ),
            Op::Sub => (
                quote! {Sub},
                quote! {SubAssign},
                quote! {sub},
                quote! {sub_assign},
            ),
            Op::Mul => (
                quote! {Mul},
                quote! {MulAssign},
                quote! {mul},
                quote! {mul_assign},
            ),
            Op::Div => (
                quote! {Div},
                quote! {DivAssign},
                quote! {div},
                quote! {div_assign},
            ),
            Op::Mod => (
                quote! {Rem},
                quote! {RemAssign},
                quote! {rem},
                quote! {rem_assign},
            ),
            Op::Pow => (
                quote! {Pow},
                quote! {PowAssign},
                quote! {pow},
                quote! {pow_assign},
            ),
        }
    }
}
fn impl_lower_ops(ty: TokenStream, oty: TokenStream, ops: Op) -> TokenStream {
    let (op, op_assign, fun, fun_assign) = ops.get();
    let i = get_impl();
    let t = get_type(ty.clone());
    let assign = if matches!(ops, Op::Pow) {
        quote! {}
    } else {
        quote! {
            #i #op<#oty> for #t {
                type Output = Self;
                fn #fun(mut self, rhs: #oty) -> Self::Output {
                    #op_assign::#fun_assign(&mut self, rhs);
                    self
                }
            }
        }
    };
    quote! {
        #assign
        #i #op<#t> for #oty {
            type Output = #t;
            fn #fun(self, rhs: #t) -> Self::Output {
                match rhs {
                    Number::Value(b) => #op::#fun(self, b).into(),
                    #[cfg(feature = "list")]
                    Number::List(mut b) => {
                        b.iter_mut().for_each(|b| {
                            let old = std::mem::replace(b, Number::Value(<#ty>::from(0).into()));
                            *b = #op::#fun(self.clone(), old);
                        });
                        Number::List(b)
                    }
                    #[cfg(feature = "units")]
                    Number::Units(u) => {todo!()}
                }
            }
        }
        #i #op_assign<#oty> for #t {
            fn #fun_assign(&mut self, rhs: #oty) {
                match self {
                    Self::Value(a) => #op_assign::#fun_assign(a, rhs.clone()),
                    #[cfg(feature = "list")]
                    Self::List(a) => a
                        .iter_mut()
                        .for_each(|a| #op_assign::#fun_assign(a, rhs.clone())),
                    #[cfg(feature = "units")]
                    Self::Units(u) => {todo!()}
                }
            }
        }
    }
}
fn impl_ops(ty: TokenStream, ops: Op) -> TokenStream {
    let lower = impl_lower_ops(ty.clone(), ty.clone(), ops);
    let (op, op_assign, fun, fun_assign) = ops.get();
    let i = get_impl();
    let t = get_type(ty.clone());
    let assign = if matches!(ops, Op::Pow) {
        quote! {}
    } else {
        quote! {
            #i #op<Self> for #t {
                type Output = Self;
                fn #fun(mut self, rhs: Self) -> Self::Output {
                    #op_assign::#fun_assign(&mut self, rhs);
                    self
                }
            }
        }
    };
    quote! {
        #assign
        #i #op_assign<Self> for #t {
            fn #fun_assign(&mut self, rhs: Self) {
                match (self, rhs) {
                    (Self::Value(a), Self::Value(b)) => #op_assign::#fun_assign(a, b),
                    #[cfg(feature = "list")]
                    (Self::List(a), Self::Value(b)) => a
                        .iter_mut()
                        .for_each(|a| #op_assign::#fun_assign(a, b.clone())),
                    #[cfg(feature = "list")]
                    (s @ Self::Value(_), mut r @ Self::List(_)) => {
                        std::mem::swap(s, &mut r);
                        let (Self::List(a), Self::Value(b)) = (s, r) else {
                            unreachable!()
                        };
                        a.iter_mut().for_each(|a| {
                            let old = std::mem::replace(a, Number::Value(#ty::from(0).into()));
                            *a = #op::#fun(b.clone(), old)
                        })
                    }
                    #[cfg(feature = "list")]
                    (Self::List(a), Self::List(b)) => a
                        .iter_mut()
                        .zip(b.into_iter())
                        .for_each(|(a, b)| #op_assign::#fun_assign(a, b)),
                    #[cfg(feature = "units")]
                    _ => {todo!()}
                }
            }
        }
        #lower
    }
}
#[proc_macro]
pub fn generate_lower(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = input.into_iter();
    let ty: TokenStream = [input.next().unwrap()]
        .into_iter()
        .collect::<proc_macro::TokenStream>()
        .into();
    let oty: TokenStream = [input.nth(1).unwrap()]
        .into_iter()
        .collect::<proc_macro::TokenStream>()
        .into();
    let list = [
        impl_lower_ops(ty.clone(), oty.clone(), Op::Add),
        impl_lower_ops(ty.clone(), oty.clone(), Op::Sub),
        impl_lower_ops(ty.clone(), oty.clone(), Op::Div),
        impl_lower_ops(ty.clone(), oty.clone(), Op::Mul),
        impl_lower_ops(ty.clone(), oty.clone(), Op::Mod),
        impl_lower_ops(ty.clone(), oty.clone(), Op::Pow),
    ];
    quote! {
        #(#list)*
    }
    .into()
}
#[proc_macro]
pub fn generate_types(ty: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ty: TokenStream = ty.into();
    let i = get_impl();
    let g = get_impl_generic();
    let t = get_type(ty.clone());
    let list = [
        impl_ops(ty.clone(), Op::Add),
        impl_ops(ty.clone(), Op::Sub),
        impl_ops(ty.clone(), Op::Div),
        impl_ops(ty.clone(), Op::Mul),
        impl_ops(ty.clone(), Op::Mod),
        impl_ops(ty.clone(), Op::Pow),
    ];
    quote! {
        #g From<T> for #t
        where
            #ty: From<T>,
        {
            fn from(value: T) -> Self {
                Self::Value(value.into())
            }
        }
        #i Default for #t {
            fn default() -> Self {
                Self::Value(#ty::default().into())
            }
        }
        #i Sum for #t {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::default(), |sum, s| sum + s)
            }
        }
        #i Product for #t {
            fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::from(1.0), |sum, s| sum * s)
            }
        }
        #i NegAssign for #t {
            fn neg_assign(&mut self) {
                match self {
                    Self::Value(a) => a.neg_assign(),
                    #[cfg(feature = "list")]
                    Self::List(a) => a.iter_mut().for_each(|a| a.neg_assign()),
                    #[cfg(feature = "units")]
                    Self::Units(u) => {todo!()}
                }
            }
        }
        #i Neg for #t {
            type Output = Self;
            fn neg(mut self) -> Self {
                self.neg_assign();
                self
            }
        }
        #i Display for #t {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    Self::Value(a) => write!(f, "{}", a),
                    #[cfg(feature = "list")]
                    Self::List(a) => {
                        write!(f, "[")?;
                        let mut first = true;
                        for a in a.iter() {
                            if !first {
                                write!(f, ",")?
                            } else {
                                first = false;
                            }
                            write!(f, "{}", a)?
                        }
                        write!(f, "]")
                    }
                    #[cfg(feature = "units")]
                    Self::Units(u) => {todo!()}
                }
            }
        }
        #(#list)*
    }
    .into()
}
fn ops() -> [TokenStream; 6] {
    [
        quote! {Add},
        quote! {Sub},
        quote! {Mul},
        quote! {Div},
        quote! {Pow},
        quote! {Rem},
    ]
}
fn ops_assign() -> [TokenStream; 6] {
    [
        quote! {AddAssign},
        quote! {SubAssign},
        quote! {MulAssign},
        quote! {DivAssign},
        quote! {PowAssign},
        quote! {RemAssign},
    ]
}
fn primatives() -> [TokenStream; 17] {
    [
        quote! {i8},
        quote! {i16},
        quote! {i32},
        quote! {i64},
        quote! {i128},
        quote! {isize},
        quote! {u8},
        quote! {u16},
        quote! {u32},
        quote! {u64},
        quote! {u128},
        quote! {usize},
        quote! {f16},
        quote! {f32},
        quote! {f64},
        quote! {f128},
        quote! {bool},
    ]
}
fn float_trait() -> TokenStream {
    let primatives = primatives();
    let ops = ops();
    let ops_assign = ops_assign();
    let req = quote! {
        FloatFunctions<F> +
        FloatFunctionsMut<F> +
        FloatTrait<F> +
        From<F> +
        From<Constant> +
        #(From<#primatives> +)*
        #(#ops<Self, Output = Self> +)*
        #(for<'a> #ops<&'a Self, Output = Self> +)*
        #(#ops_assign<Self> +)*
        PartialEq +
        Neg<Output = Self> +
        Clone +
        Send +
        Sync +
        Sized +
        Debug +
        Display +
        'static +
    };
    quote! {
        pub trait FloatType<F>: #req {}
    }
}
fn real_trait() -> TokenStream {
    let req = quote! {
        FloatType<Self> +
        RealTrait<Self> +
        PartialOrd<Self> +
    };
    quote! {
        pub trait RealType: #req {}
    }
}
fn complex_trait() -> TokenStream {
    let primatives = primatives();
    let f = [quote! {F}, quote! {Constant}];
    let prim_prim = primatives.iter().chain(f.iter()).flat_map(|a| {
        primatives.iter().chain(f.iter()).map(move |b| {
            quote! {
                From<(#a, #b)>
            }
        })
    });
    let prim_ref = primatives.iter().chain(f.iter()).map(move |b| {
        quote! {
            for<'a> From<(&'a F, #b)>
        }
    });
    let ref_prim = primatives.iter().chain(f.iter()).map(move |a| {
        quote! {
            for<'a> From<(#a, &'a F)>
        }
    });
    let ops = ops();
    let ops_assign = ops_assign();
    let req = quote! {
        ComplexFunctions<F> +
        ComplexFunctionsMut<F> +
        FloatType<F> +
        ComplexTrait<F> +
        #(#prim_prim +)*
        #(#prim_ref +)*
        #(#ref_prim +)*
        #(#ops<F, Output = Self> +)*
        #(for<'a> #ops<&'a F, Output = Self> +)*
        #(#ops_assign<F> +)*
    };
    quote! {
        pub trait ComplexType<F>: #req {}
    }
}
#[proc_macro]
pub fn generate_traits(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let real = real_trait();
    let float = float_trait();
    let complex = complex_trait();
    quote! {
        #real
        #float
        #complex
    }
    .into()
}
