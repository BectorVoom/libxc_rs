//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 899/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk899<F: Float>(t3798: F, t5234: F, t1824: F, t3792: F, t12345: F, t1831: F, t3865: F, t12189: F, t1811: F, t1815: F, t3862: F, t3802: F) -> (F, F, F, F, F, F, F) {
    let t16288 = t5234 * t3798;
    let t16311 = t1824 * t3792;
    let t16317 = t12345 * t1831;
    let t16336 = t5234 * t3865;
    let t16341 = t12189 * t1811;
    let t16350 = t1815 * t3862;
    let t16394 = t5234 * t3802;
    (t16288, t16311, t16317, t16336, t16341, t16350, t16394)
}
