//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 932/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk932<F: Float>(t13396: F, t226: F, t13263: F, t4282: F, t2633: F, t9632: F, t2732: F, t4234: F, t2679: F, t4295: F, t1519: F, t2627: F, t10076: F, t1510: F, t13381: F, t13385: F, t13388: F, t13390: F, t13393: F, t2617: F, t2729: F, t2733: F, t2736: F, t4166: F, t4281: F, t4291: F, t4292: F, t4296: F, t812: F) -> (F,) {
    let t13397 = t226 * t13396;
    let t13398 = t4282 * t13263;
    let t13401 = t4282 * t2633;
    let t13404 = t4282 * t9632;
    let t13407 = t2732 * t4234;
    let t13414 = t4295 * t2679;
    let t13416 = t2627 * t1519;
    let t13417 = t13416 * t2633;
    let t13423 = t10076 * t1510;
    let t13425 = -2.0 * t13381 * t4291 + 4.0 * t13385 * t4281 - t13388 * t4291 - 2.0 * t13390 * t4292 + 4.0 * t13393 * t4281 - 6.0 * t13397 * t13398 + 6.0 * t13401 * t4281 + 2.0 * t13404 * t4281 - 2.0 * t13407 * t812 - t13414 * t812 + 2.0 * t13417 * t812 - t13423 * t812 - 2.0 * t2617 * t4296 + 2.0 * t2729 * t4166 - 2.0 * t2733 * t4166 - t2736 * t4166;
    (t13425,)
}
