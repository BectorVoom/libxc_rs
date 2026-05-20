//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1212/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1212<F: Float>(t23384: F, t32987: F, t30781: F, t7560: F, t225: F, t33007: F, t33005: F, t1052: F, t1065: F, t1066: F, t113296: F, t113468: F, t113600: F, t119351: F, t1599: F, t1634: F, t1956: F, t23346: F, t23365: F, t23369: F, t25453: F, t25778: F, t30778: F, t30861: F, t30899: F, t3174: F, t32964: F, t32992: F, t349: F, t388: F, t4542: F, t4660: F, t6687: F, t6771: F, t6816: F, t7600: F, t89620: F, t986: F) -> F {
    let t119495 = t23384 * t32987;
    let t119503 = t7560 * t30781;
    let t119523 = t33007 * t225;
    let t119527 = t33005 * t225;
    let t119529 = t349 * t119351 * t388 + F::new(2.0) * t1052 * t3174 * t30899 * t1634 - F::cast_from(0.14621636149762012769e-1_f64) * t23346 * t32987 + F::cast_from(0.18277045187202515961e-2_f64) * t119495 + F::new(2.0) * t4660 * t30778 + F::new(2.0) * t1052 * t3174 * t32964 * t1065 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t986 * t119503 - F::new(2.0) * t25778 * t6816 + F::new(4.0) * t6771 * t25453 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t23365 * t32992 - F::cast_from(0.54831135561607547883e-2_f64) * t113468 - F::new(2.0) * t89620 * t1956 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t4542 * t30861 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t1599 * t113296 - t119523 * t1066 + F::new(4.0) * t23369 * t7600 - t119527 * t1066 + t113600;
    t119529
}
