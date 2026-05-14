//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 709/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk709<F: Float>(t1289: F, t2459: F, t581: F, t2457: F, t128: F, t2464: F, t835: F, t3431: F, t836: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3748 = t2459 * t1289;
    let t3749 = t3748 * t581;
    let t3750 = t2457 * t3749;
    let t3751 = t128 * t3750;
    let t3753 = t2464 * t1289;
    let t3754 = t3753 * t581;
    let t3755 = t835 * t3754;
    let t3756 = t128 * t3755;
    let t3758 = t836 * t3431;
    (t3748, t3749, t3750, t3751, t3753, t3754, t3755, t3756, t3758)
}
