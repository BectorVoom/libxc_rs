//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 711/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk711<F: Float>(t9727: F, t9780: F, t9789: F, t9793: F, t9797: F, t9863: F, t9865: F, t9867: F, t9870: F, t9872: F, t9876: F, t118: F, t753: F, t2375: F, t2371: F, t677: F) -> (F, F, F) {
    let t9877 = t9727 + t9863 + t9780 + t9865 - t9867 - t9789 + t9870 + t9872 + t9793 + t9797 - t9876;
    let t9879 = t753 * t118;
    let t9880 = t9879 * t2375;
    let t9881 = 0.32530743900905219526e-1 * t9880;
    let t9882 = t677 * t2371;
    (t9877, t9881, t9882)
}
