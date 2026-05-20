//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2523/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2523<F: Float>(t50853: F, t43768: F, t43770: F, t44027: F, t50846: F, t50848: F, t50851: F, t50859: F, t50863: F, t50867: F, t50871: F, t50875: F) -> F {
    let t51151 = F::cast_from(0.27385555555555555556e0_f64) * t50853;
    let t51159 = -F::cast_from(0.24342716049382716049e0_f64) * t50846 - F::cast_from(0.16431333333333333333e0_f64) * t50848 + F::cast_from(0.82156666666666666667e-1_f64) * t50851 + t51151 + F::cast_from(0.54771111111111111111e-1_f64) * t43768 - F::cast_from(0.32862666666666666666e0_f64) * t43770 + t44027 - F::cast_from(0.27385555555555555556e-1_f64) * t50859 - F::cast_from(0.98587999999999999998e0_f64) * t50863 + F::cast_from(0.49293999999999999999e0_f64) * t50867 + F::new(0.147882e1) * t50871 + F::cast_from(0.16431333333333333333e0_f64) * t50875;
    t51159
}
