/**
Create a unit system.

As this macro performs various imports, it is strongly recommended that you call it
inside of its own module.

Note that it has some imports from the peano crate, so it must be included.

# Example
```rust
#[macro_use]
extern crate dimensioned;

mod fruit {
    make_units! {
        Fruit, Unitless, one;
        base {
            Apple, apple, a;
            Banana, banana, b;
            Cucumber, cuke, c;
            Mango, mango, m;
            Watermelon, watermelon, w;
        }
        derived {
        }
    }
}
use fruit::{apple, banana, cuke, mango, watermelon};

fn main() {
    let fruit_salad = apple * banana * mango * mango * watermelon;
    println!("Mmmm, delicious: {}", fruit_salad);
    assert_eq!(format!("{}", fruit_salad), "1 a*b*m^2*w");
}
```

The line `Fruit, Unitless, one;` names the unit system `Fruit`, names its type for
unitless data `Unitless` and creates the corresponding constant `one`.

The `base` block is used to define the base units of this system. The line `Apple,
apple, a;` creates the unit `Apple`, the corresponding constant `apple`, and will use
the token "a" to print `Apple`s.

The `derived` block is not yet implemented, but will be used to define derived units and
constants.
*/
#[macro_export]
macro_rules! make_units {
    ($System:ident, $Unitless:ident, $one:ident;
     base {
         $($Type:ident, $constant:ident, $print_as:ident;)+
     }
     derived {
         $($derived_constant:ident: $Derived:ident = ($($derived_rhs:tt)+);)*
     } ) => (
        make_units_adv!{
            $System, $Unitless, $one, f64, 1.0;
            base {
                $(P1, $Type, $constant, $print_as;)*
            }
            derived {
                $($derived_constant: $Derived = ($($derived_rhs)+);)*
            }
        }

        );
}
/**
Create a unit system with more flexibility than `make_units!()`.

As this macro performs various imports, it is strongly recommended that you call it
inside of its own module.

# Example

Here we define the **CGS** unit system.

```rust
#[macro_use]
extern crate dimensioned;

mod cgs {
    make_units_adv! {
        CGS, Unitless, one, f64, 1.0;
        base {
            P2, Centimeter, cm, cm;
            P2, Gram, g, g;
            P1, Second, s, s;
        }
        derived {
        }
    }
}

# fn main() {
# }
```

The line `CGS, Unitless, one, f64, 1.0;` names the unit system `CGS`, names its type for
unitless data `Unitless` and creates the corresponding constant `one`. It also states
that all constants will be of type `Dim<D, f64>` and will be initialized to a value of
`1.0`.

Once associated constants hit, `std::num::One` will be used to determine the initalize value.

The `base` block is used to define the base units of this system. The line `P2,
Centimeter, cm, cm;` creates the unit `Centimeter`, the corresponding constant `cm`, and
will use the token "cm" to print `Centimeter`s. It also states that square roots will be
allowed for `Centimeter`s; the `P2` is the type number for 2 and dictates the highest
root allowed. You will almost always want this to be `P1`. For `CGS`, though, some
derived units are defined in terms of square roots of base units, so they are necessary
to allow.

The `derived` block is not yet implemented, but will be used to define derived units and
constants.

*/
#[macro_export]
macro_rules! make_units_adv {
    ($System:ident, $Unitless:ident, $one:ident, $OneType:ident, $val:expr;
     base {
         $($Root:ident, $Type:ident, $constant:ident, $print_as:ident;)+
     }
     derived {
         $($derived_constant:ident: $Derived:ident = ($($derived_rhs:tt)+);)*
     } ) => (
        #[allow(unused_imports)]
        use $crate::{Z0, P1, P2, P3, P4, P5, P6, P7, P8, P9, N1, N2, N3, N4, N5, N6, N7, N8, N9};
        use $crate::Integer;
        use $crate::{Dimension, Dimensionless, Dim, Pow, Root, Recip, FmtDim};
        use $crate::reexported::ops::{Add, Neg, Sub, Mul, Div};
        use $crate::reexported::marker::PhantomData;
        use $crate::reexported::fmt;

        #[derive(Copy, Clone)]
        pub struct $System<$($Type: Integer = Z0),*> {
            $($constant: PhantomData<$Type>),*
        }
        impl<$($Type: Integer),*> Dimension for $System<$($Type),*> {}

        // using $Type and $constant for these traits is confusing. It should really be $Type_Left
        // and $Type_Right or something, but that is not yet supported by Rust
        #[allow(non_camel_case_types)]
        impl<$($Type),*, $($constant),*> Mul<$System<$($constant),*>> for $System<$($Type),*>
        where $($Type: Integer + Add<$constant>),*,
              $($constant: Integer),*,
              $(<$Type as Add<$constant>>::Output: Integer),*,
              $System<$(<$Type as Add<$constant>>::Output),*>: Dimension,
        {
            type Output = $System<$(<$Type as Add<$constant>>::Output),*>;
            fn mul(self, _: $System<$($constant),*>) -> Self::Output { unreachable!()  }
        }

        #[allow(non_camel_case_types)]
        impl<$($Type),*, $($constant),*> Div<$System<$($constant),*>> for $System<$($Type),*>
            where $($Type: Integer + Sub<$constant>),*,
                  $($constant: Integer),*,
                  $(<$Type as Sub<$constant>>::Output: Integer),*
        {
            type Output = $System<$(<$Type as Sub<$constant>>::Output),*>;
            fn div(self, _: $System<$($constant),*>) -> Self::Output { unreachable!()  }
        }

        // Note that this is backwards from the definition of `Pow`. We should be doing:
        // impl<$($Type),*, RHS> Pow<$System<$($Type),*>> for RHS as RHS is really the exponent,
        // but it's in the place of the base. Rust won't let us do that generically, so we've
        // switched them, as the operation on dimensions is multiplication so it's commutative.
        impl<$($Type),*, RHS> Pow<RHS> for $System<$($Type),*>
            where $($Type: Integer + Mul<RHS>),*, RHS: Integer,
                  $(<$Type as Mul<RHS>>::Output: Integer),*
        {
            type Output = $System<$(<$Type as Mul<RHS>>::Output),*>;
            fn pow(_: RHS) -> Self::Output { unreachable!() }
        }

        impl<$($Type),*, RHS> Root<RHS> for $System<$($Type),*>
            where $($Type: Integer + Div<RHS>),*, RHS: Integer,
                  $(<$Type as Div<RHS>>::Output: Integer),*
        {
            type Output = $System<$(<$Type as Div<RHS>>::Output),*>;
            fn root(_: RHS) -> Self::Output { unreachable!() }
        }

        impl<$($Type),*> Recip for $System<$($Type),*> where
            $($Type: Integer + Neg),*, $(<$Type as Neg>::Output: Integer),* {
            type Output = $System<$(<$Type as Neg>::Output),*>;
            fn recip(self) -> Self::Output { unreachable!() }
        }

        impl<$($Type),*> FmtDim for $System<$($Type),*>
            where $($Type: Integer),*
        {
            fn fmt(f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                let allowed_roots = [$($Root::to_i32()),*];
                let exponents = [$($Type::to_i32()),*];
                let print_tokens = [$(stringify!($print_as)),*];

                let mut first = true;

                for ((&root, &exp), &token) in
                    allowed_roots.iter()
                    .zip(exponents.iter())
                    .zip(print_tokens.iter())
                {
                    if first {
                        first = false;
                    } else if exp != 0 {
                        write!(f, "*")?;
                    }

                    match exp {
                        0 => (),
                        _ if exp == root => write!(f, "{}", token)?,
                        _ => {
                            if exp % root == 0 {
                                write!(f, "{}^{}", token, exp/root)?
                            } else {
                                write!(f, "{}^{:.2}", token, exp as f32/root as f32)?
                            }
                        },
                    }
                }
                Ok(())
            }
        }

        pub type $Unitless = $System;
        impl Dimensionless for $Unitless {}
        #[allow(non_upper_case_globals, dead_code)]
        pub const $one: Dim<$Unitless, $OneType> = Dim($val, PhantomData);

        __make_base_types!($System, $($Type, $Root),+ |);

        $(#[allow(non_upper_case_globals, dead_code)]
          pub const $constant: Dim<$Type, $OneType> = Dim($val, PhantomData));*;

        $(pub type $Derived = unit!($($derived_rhs)+);
          #[allow(non_upper_case_globals)]
          pub const $derived_constant: Dim<$Derived, $OneType> = Dim($val, PhantomData);
        )*
        );
}

/** Counts the number of arguments its called with and gives you the total.

#Example

```rust
#[macro_use]
extern crate dimensioned as dim;

fn main() {
    let x = count_args!(a, b, cat, banana);
    assert_eq!(4, x);
}
```
*/
#[macro_export]
macro_rules! count_args {
    ($arg:ident, $($args:ident),+) => (
        1 + count_args!($($args),+);
    );
    ($arg:ident) => (
        1
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __make_base_types {
    ($System:ident, $Type:ident, $Root:ident, $($Types:ident, $Roots:ident),+ | $($Zeros:ident),*)
        => (
        pub type $Type = $System< $($Zeros,)* $Root>;
        __make_base_types!($System, $($Types, $Roots),+ | Z0 $(, $Zeros)*);
        );
    ($System:ident, $Type:ident, $Root:ident | $($Zeros:ident),*) => (
        pub type $Type = $System<$($Zeros,)* $Root>;
        );
}

/// Creates a derived unit based on existing ones.
/// Currently only supports the operations * and /.
///
/// The ideal way to create derived units is inside the make_units! macro (which calls this one),
/// but this also lets you create derived units for systems that are already defined.
///
/// # Example
/// ```rust
/// #![feature(type_macros)]
/// #[macro_use]
/// extern crate dimensioned as dim;
/// use std::ops::Div;
/// use dim::{Dim};
/// use dim::si::*;
/// type MPS = unit!(Meter / Second);
///
/// fn speed(dist: Dim<Meter, f64>, time: Dim<Second, f64>) -> Dim<MPS, f64> {
///     dist / time
/// }
///
/// fn main() {
///     let x = 5.0 * m;
///     let t = 1.0 * s;
///     let v = speed(x, t);
///     assert_eq!(v, x/t);
/// }
/// ```
#[macro_export]
macro_rules! unit {
    // { ( $($LHS:tt)+ ) } => { unit!($($LHS)+) };
    { $LHS:tt * $($RHS:tt)+ } => { <unit!($LHS) as Mul<unit!($($RHS)+)>>::Output };
    { $LHS:tt / $($RHS:tt)+ } => { <unit!($LHS) as Div<unit!($($RHS)+)>>::Output };
    { $LHS:ty } => { $LHS };
}
