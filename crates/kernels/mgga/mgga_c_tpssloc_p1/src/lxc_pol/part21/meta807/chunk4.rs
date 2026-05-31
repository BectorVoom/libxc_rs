//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2814/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2814<F: Float>(t17087: F, t225: F, t17060: F, t13050: F, t13071: F, t13072: F, t13377: F, t13460: F, t13463: F, t1492: F, t1527: F, t1528: F, t17022: F, t17049: F, t17050: F, t17057: F, t25168: F, t259: F, t2597: F, t2713: F, t2718: F, t4147: F, t4268: F, t4273: F, t46452: F, t46488: F, t47585: F, t5637: F, t798: F, t855: F, t865: F, t866: F, t9593: F) -> F {
    let t59498 = t17087 * t225;
    let t59503 = t17060 * t225;
    let t59518 = F::cast_from(2.0_f64) * t1492 * t13377 * t259 + F::cast_from(4.0_f64) * t855 * t2718 * t1527 * t13460 + F::cast_from(8.0_f64) * t13463 * t4273 - F::cast_from(12.0_f64) * t4147 * t13050 - F::cast_from(2.0_f64) * t46452 * t1528 - F::cast_from(2.0_f64) * t47585 * t1528 + F::cast_from(4.0_f64) * t855 * t2718 * t17049 * t865 - F::cast_from(2.0_f64) * t2713 * t17050 - F::cast_from(4.0_f64) * t59498 * t866 + F::cast_from(4.0_f64) * t2597 * t17057 - F::cast_from(2.0_f64) * t59503 * t866 - F::cast_from(24.0_f64) * t25168 * t46488 * t13071 + F::cast_from(4.0_f64) * t9593 * t5637 + F::cast_from(8.0_f64) * t4268 * t13072 - F::cast_from(12.0_f64) * t4268 * t13050 + F::cast_from(2.0_f64) * t798 * t17022 * t259;
    t59518
}
