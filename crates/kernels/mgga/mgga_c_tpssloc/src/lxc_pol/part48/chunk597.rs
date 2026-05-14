//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 597/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk597<F: Float>(t113: F, t8595: F, t1873: F, t7042: F, t88: F, t2039: F, t191: F, t2079: F, t192: F) -> (F, F, F, F, F, F) {
    let t8596 = t113 * t8595;
    let t8598 = 2.0 * t7042 * t1873;
    let t8601 = t88 * t1873;
    let t8603 = 2.0 * t8601 * t2039;
    let t8606 = t2079 * t191;
    let t8607 = t8606 * t192;
    (t8596, t8598, t8601, t8603, t8606, t8607)
}
