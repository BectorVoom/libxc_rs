//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 838/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk838<F: Float>(t1235: F, t5721: F, t1239: F, t1765: F, t522: F, t64: F, t234: F, t339: F) -> (F, F, F, F) {
    let t5722 = t5721 * t1235;
    let t5724 = t1765 * t1239;
    let t5725 = 7.0 / 2304.0 * t5724;
    let t5726 = t522 * t64;
    let t5728 = t339 * t5726 * t234;
    (t5722, t5725, t5726, t5728)
}
