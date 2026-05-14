//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1225/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1225<F: Float>(t4256: F, t6005: F, t938: F, t1120: F, t6504: F, t1875: F, t339: F, t4263: F) -> (F, F, F) {
    let t20831 = t938 * t6005 * t4256;
    let t20834 = t6504 * t1120;
    let t20837 = t339 * t1875 * t4263;
    (t20831, t20834, t20837)
}
