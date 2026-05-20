//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1450/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1450<F: Float>(t33245: F, t6897: F, t794: F, t1985: F, t26202: F, t31611: F, t115658: F, t120641: F, t120649: F, t1375: F, t16030: F, t1842: F, t2016: F, t24095: F, t26348: F, t26371: F, t31564: F, t31641: F, t33323: F, t3887: F, t5321: F, t6992: F, t7194: F, t7729: F, t7936: F, t8627: F, t90665: F, t93338: F) -> F {
    let t122551 = t6897 * t794 * t33245;
    let t122562 = t1985 * t31611 * t26202;
    let t122576 = -F::new(6.0) * t90665 * t33323 - t120641 - F::cast_from(0.41123351671205660912e-2_f64) * t122551 + F::new(2.0) * t16030 * t8627 + F::new(2.0) * t1375 * t3887 * t31641 * t1842 - F::cast_from(0.41123351671205660912e-2_f64) * t115658 - t93338 * t2016 - F::cast_from(0.82246703342411321825e-2_f64) * t122562 - t120649 + F::new(2.0) * t7194 * t26348 + F::new(2.0) * t7194 * t26371 + F::new(2.0) * t24095 * t7729 + F::new(2.0) * t1375 * t3887 * t7936 * t6992 + F::new(2.0) * t5321 * t31564;
    t122576
}
