//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1881/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1881<F: Float>(t23185: F, t25045: F, t82074: F, t254: F, t799: F, t23270: F, t2379: F, t25039: F, t87642: F, t1880: F, t23218: F, t25224: F) -> (F, F, F, F) {
    let t87753 = t23185 * t82074 * t25045;
    let t87755 = t799 * t254;
    let t87765 = t87642 * t23270 * t25039 * t2379;
    let t87773 = t1880 * t25224 * t23218;
    (t87753, t87755, t87765, t87773)
}
