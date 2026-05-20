//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 747/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk747<F: Float>(t1268: F, t2039: F, t2314: F, t5113: F, t671: F, t7040: F, t7042: F, t7056: F, t2094: F, t532: F, t6879: F, t6884: F) -> (F, F, F, F) {
    let t7166 = F::new(2.0) * t1268 * t7056 + F::new(2.0) * t2039 * t2314 + F::new(2.0) * t2039 * t5113 + F::new(2.0) * t671 * t7042 + t7040;
    let t7170 = t532 * t2094;
    let t7171 = t7170 * t6879;
    let t7174 = F::cast_from(0.38381794893125283518e-1_f64) * t6884;
    (t7166, t7170, t7171, t7174)
}
