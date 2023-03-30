#![macro_use]

pub mod test;
pub mod si;

pub trait Quantity {}
pub trait Unit {}

#[macro_export]
macro_rules! quantity {
    // rule for a base quantity
    (name: $n:ident, unit: $u:ident) => {
        $crate::impl_quantity_and_unit!($n, $u);
    };

    // derived quantity (derived by multiplication)
    (name: $n:ident, unit: $u:ident, derive: $lhs:ident * $rhs:ident) => {
        // quantity + unit
        $crate::impl_quantity_and_unit!($n, $u);
        
        // operators
        $crate::impl_mul!($lhs * $rhs = $n);
        $crate::impl_div!($n / $rhs = $lhs);
        $crate::impl_div!($n / $lhs = $rhs);
    };
    
    // derived quantity (derived by division)
    (name: $n:ident, unit: $u:ident, derive: $lhs:ident / $rhs:ident) => {
        $crate::impl_quantity_and_unit!($n, $u);
        
        // operators
        $crate::impl_div!($lhs / $rhs = $n);
        $crate::impl_div!($lhs / $n = $rhs);
        $crate::impl_mul!($n * $rhs = $lhs);
    };
}

#[macro_export]
macro_rules! impl_quantity_and_unit {
    ($quantity_name:ident, $unit_name:ident) => {
        pub struct $quantity_name(pub f64);
        impl $crate::Quantity for $quantity_name {}
        impl $quantity_name {
            pub fn new(val: f64) -> Self {
                Self(val)
            }
        }
        // unit
        pub struct $unit_name;
        impl $crate::Unit for $unit_name {}
    };
}

#[macro_export]
macro_rules! impl_mul {
    ($lhs:ident * $rhs:ident = $out:ident) => {
        impl std::ops::Mul<$rhs> for $lhs {
            type Output = $out;
            
            fn mul(self, rhs: $rhs) -> Self::Output {
                $out(self.0 * rhs.0)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_div {
    ($lhs:ident / $rhs:ident = $out:ident) => {
        impl std::ops::Div<$rhs> for $lhs {
            type Output = $out;

            fn div(self, rhs: $rhs) -> Self::Output {
                $out(self.0 / rhs.0)
            }
        }
    };
}
