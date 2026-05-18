//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1416/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1416<F: Float>(t1825: F, t22633: F, t6976: F, t96951: F, t26421: F, t6415: F, t26395: F, t6347: F, t6637: F, t6888: F, t1799: F, t26331: F, t26446: F) -> (F, F, F, F) {
    let t107339 = t22633 * t6976 * t96951 * t1825;
    let t107343 = t22633 * t6976 * t26421 * t6415;
    let t107348 = t6888 * t6637 * t26395 * t6347;
    let t107353 = t26331 * t26446 * t96951 * t1799;
    (t107339, t107343, t107348, t107353)
}
