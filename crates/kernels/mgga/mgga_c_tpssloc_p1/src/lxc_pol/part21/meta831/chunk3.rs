//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2931/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2931<F: Float>(t10160: F, t1052: F, t13736: F, t13743: F, t14526: F, t14545: F, t14549: F, t14555: F, t14659: F, t1603: F, t17575: F, t17583: F, t17588: F, t18062: F, t3166: F, t3169: F, t3174: F, t3176: F, t3206: F, t388: F, t4557: F, t4660: F, t4665: F, t4694: F, t5848: F, t5943: F, t5944: F) -> F {
    let t61048 = F::new(2.0) * t1052 * t3174 * t3206 * t5943 + F::new(2.0) * t14526 * t1603 * t388 + t3166 * t388 * t5848 - F::new(2.0) * t10160 * t5944 - F::new(12.0) * t13736 * t4557 - F::new(12.0) * t13736 * t4660 + F::new(8.0) * t13743 * t4557 + F::new(8.0) * t13743 * t4660 + F::new(8.0) * t14545 * t4665 + F::new(4.0) * t14549 * t4557 + F::new(4.0) * t14549 * t4660 - F::new(4.0) * t14555 * t4694 - F::new(2.0) * t14659 * t4660 + F::new(2.0) * t17575 * t3176 + F::new(8.0) * t17583 * t3169 + F::new(4.0) * t17588 * t3176 + F::new(4.0) * t18062 * t3169;
    t61048
}
