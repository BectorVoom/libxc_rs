//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1863/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1863<F: Float>(t13193: F, t6621: F, t13198: F, t23097: F, t232: F, t46565: F, t815: F, t46644: F, t25135: F, t838: F, t2693: F, t7503: F) -> (F, F, F, F, F, F) {
    let t87389 = t6621 * t13193;
    let t87391 = t6621 * t13198;
    let t87395 = t23097 * t815 * t46565 * t232;
    let t87399 = t23097 * t815 * t46644 * t232;
    let t87401 = t25135 * t838;
    let t87403 = t7503 * t2693;
    (t87389, t87391, t87395, t87399, t87401, t87403)
}
