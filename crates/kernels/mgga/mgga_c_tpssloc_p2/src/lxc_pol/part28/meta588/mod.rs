//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta588 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1881;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1882;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta588<F: Float>(t23185: F, t25045: F, t82074: F, t254: F, t799: F, t23270: F, t2379: F, t25039: F, t87642: F, t1880: F, t23218: F, t25224: F, t6562: F, t6572: F, t86893: F, t23171: F, t23228: F, t7488: F, t214: F, t4265: F, t25055: F, t81591: F, t25217: F, t6547: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t87753, t87755, t87765, t87773) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1881::<F>(t23185, t25045, t82074, t254, t799, t23270, t2379, t25039, t87642, t1880, t23218, t25224);
        let (t87776, t87779, t87782, t87784, t87786, t87796) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1882::<F>(t6562, t6572, t86893, t23171, t23228, t7488, t214, t4265, t1880, t25055, t81591, t25217, t6547);
    (t87753, t87755, t87765, t87773, t87776, t87779, t87782, t87784, t87786, t87796)
}
