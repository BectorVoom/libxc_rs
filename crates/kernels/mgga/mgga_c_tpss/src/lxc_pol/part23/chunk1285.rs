//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1285/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1285<F: Float>(t18246: F, t35525: F, t1398: F, t2829: F, t19809: F, t61703: F, t44329: F, t1364: F, t1692: F, t1713: F, t17929: F, t18254: F, t19816: F, t20048: F, t2439: F, t5590: F, t6149: F, t64289: F, t64946: F, t64950: F, t64954: F, t64958: F, t64966: F, t64969: F, t64972: F, t64976: F) -> (F,) {
    let t64979 = t18246 * t35525;
    let t64982 = t2829 * t1398;
    let t64986 = t61703 * t19809;
    let t64989 = t18246 * t44329;
    let t64992 = t2829 * t1364;
    let t64996 = -t1692 * t5590 * t64946 / 2.0 + 3.0 / 2.0 * t2439 * t1713 * t64950 - t1692 * t5590 * t64954 + 2.0 * t19816 * t64958 + 3.0 / 2.0 * t2439 * t6149 * t18254 + 2.0 * t64289 * t20048 + 3.0 * t17929 * t64966 - 3.0 / 2.0 * t17929 * t64969 - 3.0 * t17929 * t64972 - 3.0 * t17929 * t64976 - 3.0 / 2.0 * t17929 * t64979 - t1692 * t5590 * t64982 / 2.0 - 3.0 * t17929 * t64986 - 3.0 * t17929 * t64989 + 3.0 / 2.0 * t2439 * t1713 * t64992;
    (t64996,)
}
