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
    let t59518 = F::new(2.0) * t1492 * t13377 * t259 + F::new(4.0) * t855 * t2718 * t1527 * t13460 + F::new(8.0) * t13463 * t4273 - F::new(12.0) * t4147 * t13050 - F::new(2.0) * t46452 * t1528 - F::new(2.0) * t47585 * t1528 + F::new(4.0) * t855 * t2718 * t17049 * t865 - F::new(2.0) * t2713 * t17050 - F::new(4.0) * t59498 * t866 + F::new(4.0) * t2597 * t17057 - F::new(2.0) * t59503 * t866 - F::new(24.0) * t25168 * t46488 * t13071 + F::new(4.0) * t9593 * t5637 + F::new(8.0) * t4268 * t13072 - F::new(12.0) * t4268 * t13050 + F::new(2.0) * t798 * t17022 * t259;
    t59518
}
