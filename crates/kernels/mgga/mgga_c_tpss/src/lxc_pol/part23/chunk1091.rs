//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1091/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1091<F: Float>(t33: F, t259: F, t479: F, t10937: F, t12277: F, t12649: F, t1006: F, t10353: F, t10947: F, t10948: F, t10950: F, t1157: F, t1289: F, t1402: F, t1497: F, t1594: F, t1992: F, t2445: F, t2829: F, t3158: F, t3431: F, t3735: F, t4333: F, t481: F, t57: F, t581: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t12651 = piecewise3(t480, t12277 + t12649, t10937);
    let t12663 = piecewise3(t386, t10937 * t33 / 2.0 + t3735 * t1006 + t1402 * t2829 / 2.0 + t2445 * t1497 / 2.0 - t10947 - t10948 + t10950, t12651 * t57 / 2.0 - t4333 * t581 - t1594 * t1992 / 2.0 - t3158 * t1289 / 2.0 - t1157 * t3431 - t481 * t10353 / 2.0);
    (t12663,)
}
