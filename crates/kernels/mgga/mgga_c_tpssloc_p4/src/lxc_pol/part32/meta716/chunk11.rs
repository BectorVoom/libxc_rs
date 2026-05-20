//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2270/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2270<F: Float>(t28264: F, t6547: F, t225: F, t28282: F, t17022: F, t1880: F, t214: F, t258: F, t28272: F, t6562: F, t794: F, t25224: F, t25341: F, t6552: F) -> (F, F, F, F, F) {
    let t99003 = t6547 * t28264;
    let t99010 = t28282 * t225;
    let t99019 = t1880 * t214 * t17022 * t225 * t258;
    let t99022 = t6562 * t794 * t28272;
    let t99033 = t6552 * t25224 * t25341;
    (t99003, t99010, t99019, t99022, t99033)
}
