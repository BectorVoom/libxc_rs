//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1215/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1215<F: Float>(t20799: F, t20943: F, t118: F, t1663: F, t1899: F, t19324: F, t19326: F, t19329: F, t19336: F, t19338: F, t19340: F, t19436: F, t19438: F, t19440: F, t19443: F, t19452: F, t19573: F, t4541: F, t6058: F) -> (F, F) {
    let t20944 = t20799 + t20943;
    let t20946 = -t118 * t20944 + t1663 * t6058 + t1899 * t4541 - t19324 - t19326 - t19329 - t19336 - t19338 - t19340 - t19436 - t19438 - t19440 - t19443 + t19452 + t19573;
    (t20944, t20946)
}
