//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1235/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1235<F: Float>(t30: F, t4806: F, t1288: F, t1398: F, t4802: F, t1692: F, t1713: F, t17929: F, t18052: F, t19802: F, t21256: F, t21263: F, t21266: F, t21270: F, t21345: F, t2439: F, t3552: F, t4578: F, t5590: F, t6120: F, t6149: F, t6153: F) -> (F, F, F, F) {
    let t21353 = t30 * t4806;
    let t21356 = t1288 * t1398;
    let t21359 = t30 * t4802;
    let t21366 = 3.0 * t3552 * t21256 + 3.0 * t2439 * t6149 * t6120 - 3.0 * t17929 * t21263 + 3.0 * t2439 * t1713 * t21266 + 3.0 / 2.0 * t2439 * t1713 * t21270 + t1692 * t21345 * t30 / 2.0 - t1692 * t19802 * t6153 + t1692 * t6149 * t1288 + t1692 * t18052 * t21353 - t1692 * t5590 * t21356 - t1692 * t5590 * t21359 / 2.0 + t1692 * t1713 * t4578 / 2.0;
    (t21353, t21356, t21359, t21366)
}
