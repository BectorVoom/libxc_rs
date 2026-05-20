//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1797/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1797<F: Float>(t25249: F, t776: F, t25248: F, t25038: F, t7510: F, t814: F, t829: F, t7528: F, t794: F, t6562: F, t1509: F, t1902: F) -> (F, F, F, F, F, F, F, F) {
    let t25250 = t25249 * t776;
    let t25251 = t25248 * t25250;
    let t25252 = t25038 * t25251;
    let t25255 = t814 * t7510;
    let t25256 = t25255 * t829;
    let t25258 = t794 * t7528;
    let t25259 = t6562 * t25258;
    let t25261 = t1902 * t1509;
    (t25250, t25251, t25252, t25255, t25256, t25258, t25259, t25261)
}
