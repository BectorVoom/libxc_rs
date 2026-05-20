//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2098;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2099;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta638<F: Float>(t22986: F, t25192: F, t82159: F, t254: F, t853: F, t23164: F, t23204: F, t25341: F, t12971: F, t6552: F, t6553: F, t6554: F, t776: F, t865: F, t23270: F, t25044: F, t82147: F, t13377: F, t1880: F, t214: F, t225: F, t258: F, t1887: F, t81956: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t87010, t87013, t87029, t87033) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2098::<F>(t22986, t25192, t82159, t254, t853, t23164, t23204, t25341, t12971, t6552, t6553, t6554);
        let (t87036, t87039, t87042, t87047, t87049) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2099::<F>(t776, t865, t22986, t23270, t25044, t82147, t13377, t1880, t214, t225, t258, t1887, t81956);
    (t87010, t87013, t87029, t87033, t87036, t87039, t87042, t87047, t87049)
}
