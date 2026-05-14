//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1190/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1190<F: Float>(t10514: F, t63840: F, t1398: F, t2428: F, t19817: F, t14076: F, t60960: F, t17930: F, t44329: F, t3683: F, t821: F, t2116: F, t1364: F, t1991: F, t3610: F, t580: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t63841 = t63840 * t10514;
    let t63844 = t1398 * t2428;
    let t63845 = t19817 * t63844;
    let t63847 = t60960 * t14076;
    let t63850 = t17930 * t44329;
    let t63859 = t3683 * t821;
    let t63860 = t17930 * t63859;
    let t63863 = t1398 * t2116;
    let t63864 = t17930 * t63863;
    let t63873 = t1991 * t1364;
    let t63877 = t580 * t3610;
    (t63841, t63844, t63845, t63847, t63850, t63859, t63860, t63863, t63864, t63873, t63877)
}
