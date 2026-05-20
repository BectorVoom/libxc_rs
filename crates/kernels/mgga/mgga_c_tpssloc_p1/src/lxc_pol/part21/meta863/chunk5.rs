//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3146/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3146<F: Float>(t15294: F, t15376: F, t44573: F, t44586: F, t44635: F, t44638: F, t44641: F, t52300: F, t52354: F, t52357: F, t52362: F, t52364: F, t52367: F) -> F {
    let t65161 = -F::cast_from(0.82304526748971193413e-4_f64) * t44573 + F::cast_from(0.49382716049382716047e-3_f64) * t52300 - F::cast_from(0.12345679012345679012e-3_f64) * t44586 - F::cast_from(0.59259259259259259256e-2_f64) * t15376 * t15294 + F::cast_from(0.74074074074074074072e-3_f64) * t52354 - F::cast_from(0.18518518518518518518e-3_f64) * t52357 - F::cast_from(0.55555555555555555554e-3_f64) * t52362 - F::cast_from(0.65843621399176954729e-3_f64) * t52364 + F::cast_from(0.24691358024691358024e-3_f64) * t52367 - F::cast_from(0.20576131687242798354e-3_f64) * t44635 + F::cast_from(0.6172839506172839506e-4_f64) * t44638 + F::cast_from(0.12345679012345679012e-3_f64) * t44641;
    t65161
}
