//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1036/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1036<F: Float>(t4701: F, t799: F, t750: F, t14029: F, t778: F, t1373: F, t1375: F, t14268: F, t14274: F, t14282: F, t14285: F, t222: F, t224: F, t3650: F, t3656: F, t3658: F, t3661: F, t4748: F, t4752: F, t4755: F, t776: F, t779: F) -> F {
    let t14290 = t799 * t4701;
    let t14291 = t14290 * t750;
    let t14294 = t778 * t14029;
    let t14297 = F::cast_from(6.0_f64) * t1373 * t3661 + F::cast_from(6.0_f64) * t1375 * t3650 - t14268 * t224 - F::cast_from(24.0_f64) * t14274 * t3658 + F::cast_from(60.0_f64) * t14282 * t3656 - F::cast_from(24.0_f64) * t14285 * t3656 - F::cast_from(12.0_f64) * t14291 * t3656 + F::cast_from(3.0_f64) * t14294 * t222 + F::cast_from(3.0_f64) * t4748 * t779 - F::cast_from(12.0_f64) * t4752 * t776 + F::cast_from(3.0_f64) * t4755 * t776;
    t14297
}
