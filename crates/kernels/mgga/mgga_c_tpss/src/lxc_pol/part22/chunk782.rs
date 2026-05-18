//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 782/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk782<F: Float>(t1113: F, t2785: F, t450: F, t1578: F, t1141: F, t1143: F, t1581: F, t220: F, t3124: F, t3138: F, t4293: F, t4303: F, t4307: F, t4310: F, t468: F) -> (F, F) {
    let t4314 = t2785 * t1113 * t450;
    let t4317 = t1578 * t1113;
    let t4322 = t1141 * t1143 * t4307 + t1141 * t1143 * t4310 + t1141 * t1143 * t4317 + F::new(2.0) * t1581 * t3124 * t4303 - t1581 * t3138 * t4314 + t220 * t4293 * t468;
    (t4314, t4322)
}
