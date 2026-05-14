//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1011/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1011<F: Float>(t1043: F, t11967: F, t1024: F, t1023: F, t4060: F, t1044: F, t2906: F, t4063: F, t1505: F, t2910: F, t2914: F, t1519: F, t9499: F, t2857: F, t4105: F, t11873: F) -> (F, F, F, F, F, F, F) {
    let t11968 = t11967 * t1043;
    let t11970 = 1.0 * t1024 * t11968;
    let t11971 = t4060 * t1023;
    let t11973 = 2.0 * t11971 * t1044;
    let t11975 = 1.0 * t4063 * t2906;
    let t11976 = t1505 * t2910;
    let t11978 = 0.16081979498692535067e2 * t11976 * t2914;
    let t11980 = 1.0 * t9499 * t1519;
    let t11982 = 2.0 * t2857 * t4105;
    let t11988 = 0.41203703703703703704e-2 * t11873;
    (t11970, t11973, t11975, t11978, t11980, t11982, t11988)
}
