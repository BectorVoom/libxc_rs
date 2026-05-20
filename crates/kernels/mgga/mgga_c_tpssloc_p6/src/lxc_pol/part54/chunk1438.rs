//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1438/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1438<F: Float>(t115630: F, t120628: F, t120629: F, t120633: F, t122399: F, t122406: F, t122439: F, t122470: F, t122495: F, t122515: F, t122542: F, t1375: F, t1378: F, t27132: F, t33301: F, t33316: F, t33320: F, t3758: F, t3882: F, t539: F, t568: F, t6958: F) -> F {
    let t122547 = -F::cast_from(0.49348022005446793095e-1_f64) * t122399 + t115630 + F::new(2.0) * t3758 * t33316 + F::new(2.0) * t3882 * t33316 - F::cast_from(0.82246703342411321825e-2_f64) * t122406 + t120628 + t539 * t122439 * t568 + F::new(2.0) * t6958 * t27132 + F::new(2.0) * t3882 * t33320 + F::new(2.0) * t3882 * t33301 + t120629 - t1375 * t1378 * (t122470 + t122495 + t122515 + t122542) + t120633;
    t122547
}
