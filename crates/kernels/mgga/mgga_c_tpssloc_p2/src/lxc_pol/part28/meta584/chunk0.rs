//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1873/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1873<F: Float>(t13450: F, t1888: F, t6646: F, t23110: F, t23185: F, t4292: F, t25288: F, t81591: F, t234: F, t4265: F, t6552: F, t6637: F, t776: F) -> (F, F, F, F) {
    let t87578 = t1888 * t6646 * t13450;
    let t87581 = t23185 * t23110 * t4292;
    let t87583 = t81591 * t25288;
    let t87586 = t234 * t4265;
    let t87589 = t6552 * t6637 * t87586 * t776;
    (t87578, t87581, t87583, t87589)
}
