//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1973/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1973<F: Float>(t491: F, t8034: F, t7287: F, t24567: F, t8014: F, t225: F, t8018: F, t1252: F, t15797: F, t2155: F, t24589: F, t24891: F, t27800: F, t27805: F, t27808: F, t27812: F, t27818: F, t3487: F, t4945: F, t498: F, t5055: F, t5089: F, t7283: F, t7296: F, t7351: F, t7356: F, t7392: F, t7999: F, t8088: F) -> (F, F, F, F, F) {
    let t27820 = t8034 * t491;
    let t27821 = t27820 * t7287;
    let t27826 = t24567 * t8014;
    let t27830 = t8018 * t225;
    let t27832 = -F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27800 - t3487 * t8088 - t4945 * t7392 + t27805 * t498 - t7351 * t5089 - F::cast_from(0.73108180748810063843e-2_f64) * t27808 - F::cast_from(0.21932454224643019153e-1_f64) * t7999 * t7296 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27812 - t5055 * t7392 - F::cast_from(0.91385225936012579807e-3_f64) * t24891 + F::cast_from(0.27415567780803773942e-2_f64) * t27818 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t27821 + F::new(2.0) * t5055 * t7356 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t27826 - t15797 * t2155 - t27830 * t1252;
    (t27820, t27821, t27826, t27830, t27832)
}
