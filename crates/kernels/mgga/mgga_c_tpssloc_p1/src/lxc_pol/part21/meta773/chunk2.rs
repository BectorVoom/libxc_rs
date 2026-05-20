//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2677/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2677<F: Float>(t28: F, t1081: F, t2: F, t584: F, t12000: F, t16003: F, t18196: F, t19618: F, t19623: F, t21: F, t3231: F, t3673: F, t3711: F, t39877: F, t5178: F, t53852: F, t5966: F, t6312: F, t9: F, t9212: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t56252 = t1081 * t2 * t584;
    let t56273 = piecewise3::<F>(t29, F::new(0.0), -F::new(56.0) / F::new(81.0) * t39877 * t6312 * t3673 - F::new(64.0) / F::new(27.0) * t16003 * t56252 + F::new(8.0) / F::new(27.0) * t19618 * t3231 - F::new(16.0) / F::new(9.0) * t3711 * t9 * t21 + F::new(8.0) / F::new(9.0) * t5178 * t584 - F::new(8.0) / F::new(3.0) * t5178 * t9212 + F::new(8.0) / F::new(27.0) * t12000 * t5966 * t3673 - F::new(4.0) / F::new(9.0) * t3711 * t18196 * t1081 - F::new(2.0) / F::new(9.0) * t19623 * t3231 - t53852);
    (t56252, t56273)
}
