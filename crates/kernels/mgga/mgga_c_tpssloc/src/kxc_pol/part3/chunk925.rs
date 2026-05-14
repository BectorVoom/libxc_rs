//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 925/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk925<F: Float>(t2644: F, t820: F, t1509: F, t828: F, t2647: F, t2632: F, t776: F, t1500: F, t2693: F, t4163: F, t838: F, t120: F, t4233: F, t4180: F, t4182: F, t4181: F, t9632: F) -> (F, F, F, F, F, F, F) {
    let t13222 = t2644 * t820;
    let t13223 = t1509 * t828;
    let t13224 = t13223 * t2647;
    let t13225 = t13222 * t13224;
    let t13228 = t1509 * t2632;
    let t13229 = t828 * t776;
    let t13230 = t13228 * t13229;
    let t13231 = t13222 * t13230;
    let t13234 = t1500 * t2693;
    let t13237 = 7.0 / 2304.0 * t4163 * t838;
    let t13242 = t120 * t4233;
    let t13244 = t4180 * t13242 * t4182;
    let t13248 = t4180 * t4181 * t9632;
    (t13225, t13231, t13234, t13237, t13242, t13244, t13248)
}
