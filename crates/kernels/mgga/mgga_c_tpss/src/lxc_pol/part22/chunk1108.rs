//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1108/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1108<F: Float>(t17944: F, t17971: F, t17947: F, t17950: F, t17952: F, t17957: F, t17961: F, t17965: F, t17967: F, t17969: F, t17975: F, t17977: F, t17979: F, t219: F, t5832: F, t18000: F, t1805: F, t2407: F) -> (F, F, F, F, F, F) {
    let t18737 = 35.0 / 216.0 * t17944;
    let t18746 = 119.0 / 3456.0 * t17971;
    let t18750 = t18737 + 7.0 / 36.0 * t17947 + t17950 / 8.0 - t17952 / 24.0 + t17957 / 384.0 + 7.0 / 576.0 * t17961 + t17965 / 96.0 - t17967 / 768.0 - t17969 / 768.0 + t18746 + 7.0 / 144.0 * t17975 + 5.0 / 192.0 * t17977 - t17979 / 192.0;
    let t18751 = param_beta * t18750;
    let t18753 = t5832 * t219;
    let t18767 = t18000 * t1805 * t2407;
    (t18737, t18746, t18750, t18751, t18753, t18767)
}
