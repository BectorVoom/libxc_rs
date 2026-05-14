//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1286/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1286<F: Float>(t10662: F, t20011: F, t18246: F, t44470: F, t63863: F, t10897: F, t33: F, t1006: F, t1497: F, t1692: F, t17929: F, t18043: F, t18047: F, t18247: F, t18250: F, t19670: F, t19798: F, t20018: F, t20050: F, t20058: F, t20065: F, t2439: F, t35530: F, t5586: F, t5590: F, t61269: F, t6149: F, t6207: F, t6208: F, t64284: F, t64304: F) -> (F,) {
    let t64997 = t20011 * t10662;
    let t65002 = t18246 * t44470;
    let t65013 = t18246 * t63863;
    let t65030 = t33 * t10897;
    let t65034 = 6.0 * t19670 * t64997 - 3.0 * t64284 * t18247 - 3.0 * t17929 * t65002 - 3.0 * t61269 * t20018 + 3.0 * t2439 * t5586 * t20058 + 3.0 * t2439 * t6149 * t18250 - 3.0 * t19670 * t65013 + 3.0 / 2.0 * t2439 * t18043 * t6207 + 3.0 * t35530 * t6208 - t1692 * t18047 * t20065 + t1692 * t19798 * t1006 + t1692 * t18043 * t1497 / 2.0 + t64304 - t1692 * t18047 * t20050 - t1692 * t5590 * t65030 / 2.0;
    (t65034,)
}
