//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1133/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1133<F: Float>(t2153: F, t5547: F, t2157: F, t64: F, t234: F, t339: F, t2165: F, t5550: F, t789: F) -> (F, F, F, F) {
    let t17952 = t5547 * t2153;
    let t17954 = t2157 * t64;
    let t17956 = t339 * t17954 * t234;
    let t17957 = t17956 * t2165;
    let t17960 = t339 * t5550 * t789;
    (t17952, t17954, t17957, t17960)
}
