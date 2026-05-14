//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 904/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk904<F: Float>(t3734: F, t6968: F, t6637: F, t22685: F, t6546: F, t6887: F) -> (F, F, F, F) {
    let t22747 = t6968 * t3734;
    let t22748 = t6637 * t22747;
    let t22749 = t22685 * t22748;
    let t22751 = t6546 * t6887;
    (t22747, t22748, t22749, t22751)
}
