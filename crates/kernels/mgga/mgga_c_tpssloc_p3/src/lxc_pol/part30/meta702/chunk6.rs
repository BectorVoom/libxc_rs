//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2279/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2279<F: Float>(t23384: F, t28519: F, t1052: F, t23329: F, t23346: F, t25429: F, t25453: F, t25778: F, t28510: F, t28593: F, t28679: F, t3026: F, t3174: F, t388: F, t4660: F, t4665: F, t4693: F, t5943: F, t6815: F, t7624: F, t82411: F, t83344: F, t88889: F, t88915: F, t990: F, t99099: F) -> F {
    let t99439 = t23384 * t28519;
    let t99450 = -t88889 + F::cast_from(4.0_f64) * t25778 * t4665 + t990 * t28593 * t388 + F::cast_from(4.0_f64) * t1052 * t3174 * t7624 * t4693 + F::cast_from(0.48738787165873375896e-2_f64) * t83344 - F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t23329 * t82411 * t99099 - t88915 + F::cast_from(0.21932454224643019153e-1_f64) * t23346 * t28519 - F::cast_from(0.27415567780803773942e-2_f64) * t99439 - F::cast_from(0.14621636149762012769e-1_f64) * t23346 * t28510 - t3026 * t28679 + F::cast_from(2.0_f64) * t1052 * t3174 * t6815 * t5943 + F::cast_from(4.0_f64) * t4660 * t25453;
    t99450
}
