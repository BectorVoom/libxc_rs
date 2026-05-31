//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 837/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk837<F: Float>(t4712: F, t5106: F, t671: F, t88: F, t1268: F, t1458: F, t2314: F, t4026: F, t4028: F, t4072: F, t1390: F, t1845: F) -> (F, F, F, F) {
    let t5107 = t4712 + t5106;
    let t5113 = t88 * t671;
    let t5118 = F::cast_from(2.0_f64) * t1268 * t4072 + F::cast_from(2.0_f64) * t1458 * t2314 + F::cast_from(2.0_f64) * t1458 * t5113 + F::cast_from(2.0_f64) * t4028 * t671 + t4026;
    let t5122 = t1845 * t1390;
    (t5107, t5113, t5118, t5122)
}
