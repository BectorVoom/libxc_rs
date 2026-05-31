//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2815/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2815<F: Float>(t17095: F, t225: F, t17098: F, t10049: F, t13042: F, t13059: F, t13463: F, t1528: F, t17052: F, t17064: F, t17090: F, t17092: F, t218: F, t252: F, t259: F, t2710: F, t2713: F, t2718: F, t2719: F, t2720: F, t2742: F, t2743: F, t40890: F, t4268: F, t4273: F, t4301: F, t46508: F, t5558: F, t5636: F, t5637: F, t5657: F, t59229: F, t59328: F, t855: F, t866: F) -> F {
    let t59519 = t17095 * t225;
    let t59537 = t17098 * t225;
    let t59558 = -F::cast_from(4.0_f64) * t59519 * t866 + t5558 * t2710 * t259 + F::cast_from(2.0_f64) * t10049 * t5637 - F::cast_from(2.0_f64) * t17092 * t2743 - t17052 * t2743 - F::cast_from(12.0_f64) * t2713 * t17064 + t218 * t59328 * t259 - F::cast_from(2.0_f64) * t46508 * t1528 + F::cast_from(2.0_f64) * t17052 * t2720 - F::cast_from(2.0_f64) * t59537 * t866 + F::cast_from(2.0_f64) * t855 * t2718 * t5657 * t2742 + F::cast_from(8.0_f64) * t13042 * t4273 - F::cast_from(4.0_f64) * t13463 * t4301 + F::cast_from(4.0_f64) * t4268 * t13059 + F::cast_from(24.0_f64) * t855 * t40890 * t5636 * t2719 + F::cast_from(2.0_f64) * t17090 * t2720 + t59229 * t252 * t259;
    t59558
}
