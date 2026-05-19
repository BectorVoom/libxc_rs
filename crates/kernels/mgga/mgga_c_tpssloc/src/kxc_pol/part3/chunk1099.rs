//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1099/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1099<F: Float>(t14328: F, t932: F, t4446: F, t942: F, t1573: F, t2929: F, t13716: F, t951: F, t13563: F, t13566: F, t13602: F, t10556: F, t10558: F, t10560: F, t10562: F, t10608: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F, t13613: F) -> (F, F, F, F, F) {
    let t14329 = t14328 * t932;
    let t14332 = t4446 * t942;
    let t14337 = t1573 * t2929;
    let t14344 = t13716 * t951;
    let t14352 = F::cast_from(0.41203703703703703704e-2_f64) * t13563;
    let t14353 = F::cast_from(0.12361111111111111111e-1_f64) * t13566;
    let t14354 = F::cast_from(0.61805555555555555556e-2_f64) * t13602;
    let t14363 = -t10608 - F::cast_from(0.82407407407407407407e-2_f64) * t10556 + F::cast_from(0.20601851851851851852e-2_f64) * t10558 - F::cast_from(0.61805555555555555556e-2_f64) * t10560 + F::cast_from(0.30902777777777777778e-2_f64) * t10562 - F::cast_from(0.41203703703703703704e-2_f64) * t13598 + t14352 - t14353 + t14354 - F::cast_from(0.10300925925925925926e-1_f64) * t13569 + F::cast_from(0.37083333333333333333e-1_f64) * t13572 - F::cast_from(0.12361111111111111111e-1_f64) * t13575 - F::cast_from(0.61805555555555555555e-2_f64) * t13578 - F::cast_from(0.55625000000000000001e-1_f64) * t13581 + F::cast_from(0.37083333333333333334e-1_f64) * t13584 + F::cast_from(0.18541666666666666667e-1_f64) * t13587 - F::cast_from(0.92708333333333333333e-2_f64) * t13613;
    (t14329, t14332, t14337, t14344, t14363)
}
