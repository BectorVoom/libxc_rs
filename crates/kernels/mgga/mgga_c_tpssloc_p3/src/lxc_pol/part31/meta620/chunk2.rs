//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1873/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1873<F: Float>(t1992: F, t550: F, t57499: F, t6976: F, t22704: F, t22705: F, t28163: F, t57618: F, t22881: F, t6347: F, t6637: F, t6888: F) -> (F, F, F, F) {
    let t97023 = t1992 * t6976 * t57499 * t550;
    let t97026 = t22704 * t22705 * t28163;
    let t97030 = t1992 * t6976 * t57618 * t550;
    let t97036 = t6888 * t6637 * t22881 * t6347;
    (t97023, t97026, t97030, t97036)
}
