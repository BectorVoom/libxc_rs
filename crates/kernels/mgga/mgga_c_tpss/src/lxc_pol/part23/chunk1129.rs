//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1129/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1129<F: Float>(t2162: F, t3664: F, t5754: F, t5757: F, t1760: F, t1778: F, t9909: F, t5706: F, t5710: F, t3499: F, t5532: F, t1688: F, t3166: F, t626: F, t1689: F, t7798: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14179 = t2162 * t3664;
    let t17898 = t5754 * t5757;
    let t17900 = 2.0 * t1760 * t17898;
    let t17901 = t1778 * t9909;
    let t17902 = t1760 * t17901;
    let t17904 = 6.0 * t5706 * t5710;
    let t17906 = 4.0 * t3499 * t5532;
    let t17907 = t3166 * t1688;
    let t17909 = 2.0 * t626 * t17907;
    let t17911 = 2.0 * t7798 * t1689;
    (t14179, t17898, t17900, t17901, t17902, t17904, t17906, t17907, t17909, t17911)
}
