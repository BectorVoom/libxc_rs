//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2759/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2759<F: Float>(t40: F, t12606: F, t12652: F, t1430: F, t16558: F, t16637: F, t16642: F, t2244: F, t2250: F, t4104: F, t5433: F, t5435: F, t55677: F, t55723: F, t607: F, t75: F, t767: F, zeta_threshold: F) -> F {
    let t146 = t40 <= zeta_threshold;
    let t58116 = piecewise3::<F>(t146, F::cast_from(0.0_f64), -F::cast_from(56.0_f64) / F::cast_from(81.0_f64) * t5433 * t2244 + F::cast_from(32.0_f64) / F::cast_from(27.0_f64) * t1430 * t12652 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t16637 * t2250 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t75 * t55723 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4104 * t12606 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t5435 * t2244 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t75 * t16558 * t607 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t16642 * t2250 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t767 * t55677);
    t58116
}
