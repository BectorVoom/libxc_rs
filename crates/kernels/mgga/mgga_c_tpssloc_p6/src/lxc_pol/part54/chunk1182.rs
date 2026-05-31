//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1182/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1182<F: Float>(t1375: F, t2016: F, t2092: F, t22670: F, t24095: F, t31094: F, t31103: F, t31129: F, t31140: F, t31552: F, t31555: F, t31561: F, t31564: F, t31571: F, t31573: F, t31597: F, t31601: F, t31609: F, t31613: F, t31642: F, t31666: F, t568: F, t6958: F, t6963: F, t6993: F, t7194: F, t7199: F, t7214: F) -> F {
    let t31668 = F::cast_from(0.16449340668482264365e-1_f64) * t31552 + t31094 + F::cast_from(2.0_f64) * t1375 * t31555 + F::cast_from(0.16449340668482264365e-1_f64) * t31561 + F::cast_from(2.0_f64) * t1375 * t31564 + F::cast_from(2.0_f64) * t6958 * t7199 - t31571 + t31103 - t24095 * t2016 + t31573 * t568 + t31597 + t31129 - t6958 * t7214 + F::cast_from(2.0_f64) * t1375 * t31601 + F::cast_from(2.0_f64) * t7194 * t6963 - t7194 * t6993 - F::cast_from(0.82246703342411321825e-2_f64) * t31609 - F::cast_from(0.82246703342411321825e-2_f64) * t31613 - t22670 * t2092 - t1375 * t31642 - t31140 + t31666;
    t31668
}
