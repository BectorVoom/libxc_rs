//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1064/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1064<F: Float>(t24246: F, t24250: F, t25246: F, t25259: F, t28323: F, t28331: F, t28335: F, t28339: F, t28343: F, t28347: F, t28997: F, t29000: F, t4166: F, t7837: F, t812: F) -> F {
    let t29009 = -F::cast_from(0.16449340668482264365e-1_f64) * t28323 + F::cast_from(0.16449340668482264365e-1_f64) * t25246 - F::cast_from(0.16449340668482264365e-1_f64) * t25259 - F::cast_from(0.3289868133696452873e-1_f64) * t28331 - F::new(2.0) * t812 * t28997 + t24246 + F::new(2.0) * t812 * t29000 - F::new(2.0) * t4166 * t7837 + t24250 + F::cast_from(0.16449340668482264365e-1_f64) * t28335 + F::cast_from(0.6579736267392905746e-1_f64) * t28339 + F::cast_from(0.9869604401089358619e-1_f64) * t28343 - F::cast_from(0.6579736267392905746e-1_f64) * t28347;
    t29009
}
