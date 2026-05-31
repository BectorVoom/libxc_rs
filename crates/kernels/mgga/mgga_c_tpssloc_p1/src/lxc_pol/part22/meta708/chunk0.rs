//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2302/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2302<F: Float>(t52: F, t12874: F, t12877: F, t16558: F, t16563: F, t17635: F, t20217: F, t20234: F, t2440: F, t3966: F, t40647: F, t4087: F, t5398: F, t607: F, t67060: F, t76: F, zeta_threshold: F) -> F {
    let t150 = t52 <= zeta_threshold;
    let t67082 = piecewise3::<F>(t150, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t40647 * t20234 * t607 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t16563 * t3966 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t12874 * t17635 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12877 * t5398 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t4087 * t16558 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2440 * t20217 * t607 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t76 * t67060);
    t67082
}
