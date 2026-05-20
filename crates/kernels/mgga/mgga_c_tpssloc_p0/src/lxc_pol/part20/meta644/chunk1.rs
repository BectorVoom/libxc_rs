//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2359/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2359<F: Float>(t10346: F, t13813: F, t13882: F, t13887: F, t1593: F, t1597: F, t2960: F, t343: F, t42554: F, t4546: F, t48394: F, t48397: F, t48402: F, t48407: F, t48417: F, t48421: F, t973: F) -> F {
    let t48423 = F::cast_from(0.74074074074074074072e-3_f64) * t48394 + F::cast_from(0.10288065843621399177e-3_f64) * t48397 + F::cast_from(0.13333333333333333333e-1_f64) * t2960 * t13813 - F::cast_from(0.16666666666666666666e-2_f64) * t48402 + F::cast_from(0.66666666666666666666e-2_f64) * t2960 * t13882 - F::cast_from(0.83333333333333333331e-3_f64) * t48407 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t4546 * t1597 * t10346 * t343 + F::cast_from(0.66666666666666666666e-2_f64) * t2960 * t13887 - F::cast_from(0.83333333333333333331e-3_f64) * t48417 - F::cast_from(0.12674897119341563785e-1_f64) * t42554 * t1593 + F::cast_from(0.27160493827160493826e-2_f64) * t48421;
    t48423
}
