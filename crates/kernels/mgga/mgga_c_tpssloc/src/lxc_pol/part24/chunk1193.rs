//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1193/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1193<F: Float>(t23097: F, t2628: F, t2632: F, t47320: F, t46519: F, t6605: F, t133: F, t1891: F, t6601: F, t80953: F, t46511: F, t815: F, t22816: F, t23104: F, t80967: F, t232: F, t46606: F) -> (F, F, F, F, F, F) {
    let t81728 = t23097 * t2628 * t47320 * t2632;
    let t81731 = t6605 * t2628 * t46519;
    let t81735 = t80953 * t1891 * t133 * t6601;
    let t81736 = 0.69792532988666768264e-2 * t81735;
    let t81738 = t6605 * t815 * t46511;
    let t81742 = t80967 * t1891 * t22816 * t23104;
    let t81743 = 0.43737152435318756759e-3 * t81742;
    let t81746 = t23097 * t815 * t46606 * t232;
    (t81728, t81731, t81736, t81738, t81743, t81746)
}
