//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1323/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1323<F: Float>(t10514: F, t64975: F, t18246: F, t35525: F, t1398: F, t2829: F, t19809: F, t61703: F, t44329: F, t1364: F, t10662: F, t20011: F) -> (F, F, F, F, F, F, F) {
    let t64976 = t64975 * t10514;
    let t64979 = t18246 * t35525;
    let t64982 = t2829 * t1398;
    let t64986 = t61703 * t19809;
    let t64989 = t18246 * t44329;
    let t64992 = t2829 * t1364;
    let t64997 = t20011 * t10662;
    (t64976, t64979, t64982, t64986, t64989, t64992, t64997)
}
