//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 540/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk540<F: Float>(t6793: F, t38: F, t131: F, t350: F, t1009: F, t344: F, t1014: F, t360: F, t68: F, t1059: F, t1049: F, t1948: F, t345: F, t1022: F, t1945: F, t1060: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t6794 = 1.0 / t6793;
    let t6795 = t38 * t6794;
    let t6796 = t6795 * t131;
    let t6797 = t6796 * t350;
    let t6798 = t344 * t1009;
    let t6799 = t6798 * t1014;
    let t6800 = t68 * t360;
    let t6801 = t1059 * t6800;
    let t6802 = t6799 * t6801;
    let t6805 = t1948 * t1049;
    let t6806 = t345 * t6805;
    let t6810 = t1945 * t1022;
    let t6811 = t6810 * t1060;
    (t6794, t6795, t6796, t6797, t6799, t6800, t6801, t6802, t6805, t6806, t6811)
}
