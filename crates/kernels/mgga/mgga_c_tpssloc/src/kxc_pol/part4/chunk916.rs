//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 916/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk916<F: Float>(t16673: F, t816: F, t13278: F, t1512: F, t5587: F, t9667: F, t1510: F, t4255: F, t13350: F, t120: F, t5611: F, t4180: F, t4182: F, t5527: F, t829: F, t9646: F) -> (F, F, F, F, F, F, F) {
    let t16872 = t16673 * t816;
    let t16877 = t13278 * t1512;
    let t16879 = t9667 * t5587;
    let t16887 = t1510 * t4255;
    let t16888 = t13350 * t16887;
    let t16891 = t120 * t5611;
    let t16893 = t4180 * t16891 * t4182;
    let t16896 = t120 * t5527;
    let t16898 = t9646 * t16896 * t829;
    (t16872, t16877, t16879, t16888, t16891, t16893, t16898)
}
