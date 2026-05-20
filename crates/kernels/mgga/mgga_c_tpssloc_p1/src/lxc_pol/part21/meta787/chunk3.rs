//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2741/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2741<F: Float>(t40: F, t12606: F, t12652: F, t12862: F, t16549: F, t16554: F, t16558: F, t2244: F, t2250: F, t2433: F, t40632: F, t4080: F, t5392: F, t5398: F, t55677: F, t55723: F, t607: F, t73: F, t9427: F, zeta_threshold: F) -> F {
    let t146 = t40 <= zeta_threshold;
    let t57850 = piecewise3::<F>(t146, F::new(0.0), F::new(40.0) / F::new(81.0) * t40632 * t5392 * t2244 - F::new(32.0) / F::new(27.0) * t12862 * t12652 - F::new(8.0) / F::new(27.0) * t16549 * t2250 + F::new(8.0) / F::new(9.0) * t2433 * t55723 + F::new(8.0) / F::new(9.0) * t4080 * t12606 - F::new(8.0) / F::new(27.0) * t9427 * t5398 * t2244 + F::new(8.0) / F::new(9.0) * t2433 * t16558 * t607 + F::new(4.0) / F::new(9.0) * t16554 * t2250 + F::new(4.0) / F::new(3.0) * t73 * t55677);
    t57850
}
