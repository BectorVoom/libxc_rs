//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2647/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2647<F: Float>(t28: F, t1081: F, t5966: F, t584: F, t15952: F, t15955: F, t18196: F, t19559: F, t20385: F, t20390: F, t2219: F, t3672: F, t39436: F, t5142: F, t517: F, t71090: F, zeta_threshold: F) -> (F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t73995 = t5966 * t1081;
    let t73998 = t584 * t5966;
    let t74009 = piecewise3::<F>(t29, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t39436 * t20385 * t1081 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t19559 * t2219 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t15952 * t73995 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t15955 * t73998 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t5142 * t18196 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t3672 * t20390 * t1081 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t517 * t71090);
    (t73995, t73998, t74009)
}
