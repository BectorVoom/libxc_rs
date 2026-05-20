//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1379/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1379<F: Float>(t10632: F, t10811: F, t1569: F, t1581: F, t17428: F, t21115: F, t21195: F, t2861: F, t2888: F, t41826: F, t4411: F, t49430: F, t5743: F, t5759: F, t5762: F, t59920: F, t60407: F, t69047: F, t69182: F, t76637: F, t76647: F, t76652: F, t76654: F, t76657: F, t76659: F, t76661: F, t77220: F, t77239: F, t77328: F, t932: F, t943: F, t951: F) -> F {
    let t77370 = -F::new(6.0) * t2861 * t77328 * t932 - F::cast_from(0.12304822629859687989e5_f64) * t41826 * t76637 * t10632 + F::cast_from(0.5848223622634646207e0_f64) * t943 * t77220 * t951 - t76647 + F::new(6.0) * t17428 * t5759 + F::cast_from(0.1929837539843104208e3_f64) * t60407 * t5762 + F::new(4.0) * t4411 * t21195 + F::new(4.0) * t69182 * t1569 + t76652 + t76654 - t76657 - F::new(12.0) * t59920 * t5743 - F::cast_from(0.77193501593724168322e3_f64) * t49430 * t21115 + F::cast_from(0.11579025239058625248e4_f64) * t10811 * t77239 * t2888 + F::cast_from(0.23392894490538584828e1_f64) * t69047 * t1581 - t76659 - t76661;
    t77370
}
