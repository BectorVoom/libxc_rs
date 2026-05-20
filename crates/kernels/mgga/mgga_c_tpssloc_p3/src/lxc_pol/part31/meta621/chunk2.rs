//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1876/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1876<F: Float>(t26395: F, t5187: F, t6637: F, t6888: F, t22892: F, t22893: F, t28148: F, t19761: F, t1992: F, t6976: F, t1825: F, t22633: F, t90754: F) -> (F, F, F, F) {
    let t97067 = t6888 * t6637 * t26395 * t5187;
    let t97070 = t22892 * t22893 * t28148;
    let t97079 = t1992 * t6976 * t19761;
    let t97083 = t22633 * t6976 * t90754 * t1825;
    (t97067, t97070, t97079, t97083)
}
