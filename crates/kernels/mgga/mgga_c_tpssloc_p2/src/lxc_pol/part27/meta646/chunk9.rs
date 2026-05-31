//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2228/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2228<F: Float>(t25416: F, t82431: F, t1921: F, t88804: F, t23384: F, t25811: F, t1052: F, t14526: F, t1920: F, t1927: F, t225: F, t23327: F, t23329: F, t23336: F, t23725: F, t25453: F, t25738: F, t25749: F, t25816: F, t2776: F, t3026: F, t3174: F, t345: F, t387: F, t388: F, t4552: F, t4660: F, t4693: F, t6687: F, t6768: F, t6815: F, t7553: F, t82357: F, t82402: F, t83342: F, t83344: F, t986: F) -> F {
    let t88915 = F::cast_from(0.18277045187202515961e-2_f64) * t82431 * t25416;
    let t88932 = t1921 * t88804;
    let t88937 = F::cast_from(0.18277045187202515961e-2_f64) * t23384 * t25811;
    let t88940 = F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t345 * t14526 * t225 * t387 + F::cast_from(0.26806332941230356743e-1_f64) * t83342 + F::cast_from(0.97477574331746751793e-2_f64) * t83344 + F::cast_from(0.14621636149762012769e-1_f64) * t82402 * t25816 - F::cast_from(0.3289868133696452873e-1_f64) * t1927 * t23336 * t25738 - t88915 + F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23329 * t25749 * t2776 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t82357 * t7553 + F::cast_from(2.0_f64) * t4552 * t6768 * t388 + F::cast_from(4.0_f64) * t1052 * t3174 * t6815 * t4693 + F::cast_from(4.0_f64) * t4660 * t23725 + F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t986 * t88932 + t88937 + F::cast_from(4.0_f64) * t3026 * t25453;
    t88940
}
