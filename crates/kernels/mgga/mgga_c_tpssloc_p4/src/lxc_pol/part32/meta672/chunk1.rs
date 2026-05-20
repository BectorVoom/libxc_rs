//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2106/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2106<F: Float>(t27628: F, t27634: F, t10469: F, t24719: F, t3: F, t86154: F, t2132: F, t24746: F, t95382: F, t1222: F, t27589: F, t1184: F, t1409: F) -> (F, F, F, F, F) {
    let t95387 = t27634 * t27628;
    let t95396 = t86154 * t3 * t24719 * t10469;
    let t95404 = F::cast_from(0.20186378047070195428e-3_f64) * t2132 * t95382 * t24746;
    let t95410 = t27589 * t1222 / F::new(216.0);
    let t95413 = t1409 * t1184;
    (t95387, t95396, t95404, t95410, t95413)
}
