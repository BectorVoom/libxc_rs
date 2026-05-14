//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1059/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1059<F: Float>(t1992: F, t550: F, t6976: F, t74967: F, t74937: F, t22685: F, t26395: F, t6330: F, t6637: F, t20356: F, t6968: F, t80732: F, t12250: F, t81027: F, t22897: F, t3792: F) -> (F, F, F, F, F, F) {
    let t107406 = t1992 * t6976 * t74967 * t550;
    let t107413 = t1992 * t6976 * t74937 * t550;
    let t107417 = t22685 * t6637 * t26395 * t6330;
    let t107431 = t80732 * t6637 * t6968 * t20356;
    let t107435 = t1992 * t81027 * t74967 * t12250;
    let t107439 = t1992 * t22897 * t74967 * t3792;
    (t107406, t107413, t107417, t107431, t107435, t107439)
}
