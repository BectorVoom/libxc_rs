//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 846/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk846<F: Float>(t3: F, t5760: F, t1279: F, t1786: F, t116: F, t1688: F) -> (F, F, F, F) {
    let t5761 = t3 * t5760;
    let t5766 = param_d * t5760;
    let t5771 = 3.0 * t1279 * t1786;
    let t5772 = t116 * t1688;
    (t5761, t5766, t5771, t5772)
}
