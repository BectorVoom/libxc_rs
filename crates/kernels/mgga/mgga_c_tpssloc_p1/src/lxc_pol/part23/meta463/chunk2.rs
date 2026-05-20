//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1356/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1356<F: Float>(t10214: F, t10217: F, t10278: F, t1597: F, t21444: F, t2979: F, t2980: F, t343: F, t42976: F, t4546: F, t48336: F, t48397: F, t61408: F, t61489: F, t61597: F, t61600: F, t69796: F, t69801: F, t69806: F, t75836: F, t75847: F, t973: F, t977: F) -> F {
    let t76974 = F::cast_from(0.22222222222222222221e-2_f64) * t69796 - F::cast_from(0.33333333333333333332e-2_f64) * t69801 + F::cast_from(0.11522633744855967078e-2_f64) * t69806 - F::cast_from(0.1037037037037037037e-1_f64) * t973 * t10214 * t42976 * t75836 - F::cast_from(0.33333333333333333332e-2_f64) * t973 * t4546 * t21444 * t1597 * t343 + F::cast_from(0.13333333333333333332e-1_f64) * t973 * t2979 * t10217 * t75836 + F::cast_from(0.11111111111111111111e-2_f64) * t973 * t2979 * t2980 * t75847 - F::cast_from(0.66666666666666666664e-2_f64) * t973 * t977 * t10278 * t75836 + F::cast_from(0.74074074074074074072e-3_f64) * t61408 - F::cast_from(0.12345679012345679012e-2_f64) * t48336 + F::cast_from(0.74074074074074074072e-3_f64) * t61489 - F::cast_from(0.37037037037037037036e-3_f64) * t61597 - F::cast_from(0.49382716049382716048e-3_f64) * t61600 + F::cast_from(0.41152263374485596707e-3_f64) * t48397;
    t76974
}
