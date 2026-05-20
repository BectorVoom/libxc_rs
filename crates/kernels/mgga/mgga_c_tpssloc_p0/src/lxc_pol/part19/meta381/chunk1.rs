//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1424/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1424<F: Float>(t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t43866: F, t43869: F, t43872: F, t43875: F, t43882: F, t43884: F, t43887: F, t43890: F, t43892: F) -> F {
    let t44052 = -F::cast_from(0.18257037037037037037e0_f64) * t43855 - F::cast_from(0.97370864197530864196e-1_f64) * t43857 - F::cast_from(0.97370864197530864199e0_f64) * t43859 + F::cast_from(0.54771111111111111111e0_f64) * t43861 + F::cast_from(0.10954222222222222222e1_f64) * t43863 - F::new(0.379785e1) * t43866 + F::new(0.614325e0) * t43869 + F::new(0.85451625e1) * t43872 - F::new(0.46074375e0) * t43875 - F::cast_from(0.3560484375e1_f64) * t43882 + F::new(0.1898925e1) * t43884 - F::new(0.28483875e1) * t43887 + F::cast_from(0.1151859375e0_f64) * t43890 + F::new(0.46074375e0) * t43892;
    t44052
}
