//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 712/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk712<F: Float>(t1259: F, t3384: F, t1256: F, t1266: F, t3358: F, t3360: F, t3367: F, t538: F) -> (F, F) {
    let t3385 = t1259 * t3384;
    let t3387 = 2.0 * t1256 * t3367 - t1256 * t3385 - 2.0 * t1266 * t3360 + t3358 * t538;
    (t3385, t3387)
}
