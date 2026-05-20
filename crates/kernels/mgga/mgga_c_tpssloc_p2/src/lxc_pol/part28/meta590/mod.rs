//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta590 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1885;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1886;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta590<F: Float>(t23237: F, t25341: F, t6552: F, t23204: F, t25216: F, t6562: F, t1519: F, t212: F, t23171: F, t6554: F, t23270: F, t25038: F, t258: F, t4119: F, t776: F, t25039: F, t2553: F, t25040: F, t82074: F, t87712: F, t25193: F, t81591: F, t1484: F, t2249: F, t606: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t87907, t87910, t87915, t87920) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1885::<F>(t23237, t25341, t6552, t23204, t25216, t6562, t1519, t212, t23171, t6554, t23270, t25038, t258, t4119, t776);
        let (t87924, t87927, t87931, t87953, t87957) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1886::<F>(t23270, t25038, t25039, t2553, t25040, t82074, t87712, t25193, t81591, t1484, t2249, t4119, t606);
    (t87907, t87910, t87915, t87920, t87924, t87927, t87931, t87953, t87957)
}
