//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1082/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1082<F: Float>(t1842: F, t7213: F, t3887: F, t1807: F, t7191: F, t1375: F, t16460: F, t1843: F, t2092: F, t22908: F, t22910: F, t22922: F, t22928: F, t22941: F, t24082: F, t24156: F, t24157: F, t5215: F, t5321: F, t5354: F, t568: F, t7194: F, t7199: F, t7214: F) -> (F, F, F) {
    let t27131 = t7213 * t1842;
    let t27132 = t3887 * t27131;
    let t27137 = t1807 * t7191;
    let t27141 = -t5215 * t7214 + t22908 + t22910 - t16460 * t2092 - t7194 * t5354 + t22922 + F::new(2.0) * t1375 * t27132 - t24082 * t1843 + t24156 + t24157 - F::cast_from(0.82246703342411321825e-2_f64) * t22928 + t27137 * t568 + F::new(2.0) * t5321 * t7199 - t22941;
    (t27132, t27137, t27141)
}
