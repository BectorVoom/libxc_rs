//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2314/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2314<F: Float>(t52: F, t12961: F, t1431: F, t16558: F, t16649: F, t17635: F, t20217: F, t20234: F, t2298: F, t3966: F, t4111: F, t5398: F, t607: F, t67060: F, t771: F, t78: F, zeta_threshold: F) -> F {
    let t150 = t52 <= zeta_threshold;
    let t67280 = piecewise3::<F>(t150, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t2298 * t20234 * t607 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t16649 * t3966 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1431 * t17635 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t12961 * t5398 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t4111 * t16558 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t78 * t20217 * t607 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t771 * t67060);
    t67280
}
