//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1330/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1330<F: Float>(t18246: F, t69810: F, t20047: F, t69847: F, t14245: F, t20011: F, t14029: F, t33: F, t52613: F, t1006: F, t4706: F, t14256: F) -> (F, F, F, F, F, F, F) {
    let t70890 = t18246 * t69810;
    let t70893 = t20047 * t69847;
    let t70906 = t20011 * t14245;
    let t70909 = t33 * t14029;
    let t70915 = t18246 * t52613;
    let t70923 = t1006 * t4706;
    let t70929 = t20011 * t14256;
    (t70890, t70893, t70906, t70909, t70915, t70923, t70929)
}
