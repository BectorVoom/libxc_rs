//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 822/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk822<F: Float>(t2770: F, t343: F, t2775: F, t2769: F, t40: F, t344: F, t241: F, t625: F, t281: F, t283: F, t2978: F, t340: F, t63: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10236 = t343 * t2770;
    let t10254 = t343 * t2775;
    let t10276 = t2769 * t40;
    let t10277 = F::new(1.0) / t10276;
    let t10278 = t344 * t10277;
    let t10292 = t625 * t241;
    let t10294 = t281 * t10292 * t283;
    let t10295 = F::new(20.0) / F::new(27.0) * t10294;
    let t10304 = t241 * t2978;
    let t10335 = t63 * t340;
    (t10236, t10254, t10277, t10278, t10292, t10294, t10295, t10304, t10335)
}
