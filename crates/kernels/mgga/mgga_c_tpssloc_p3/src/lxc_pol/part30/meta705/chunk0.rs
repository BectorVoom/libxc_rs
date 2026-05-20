//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2308/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2308<F: Float>(t5932: F, t6743: F, t28653: F, t82822: F, t1014: F, t1058: F, t1060: F, t11046: F, t14608: F, t1625: F, t17959: F, t18093: F, t1945: F, t23478: F, t23601: F, t23602: F, t23633: F, t25492: F, t25516: F, t25554: F, t25558: F, t25712: F, t28596: F, t28601: F, t28641: F, t3186: F, t4673: F, t6687: F, t82717: F, t89175: F, t89224: F) -> F {
    let t100204 = t6743 * t5932;
    let t100215 = t82822 * t28653;
    let t100225 = t89175 + F::new(2.0) * t3186 * t28641 * t4673 - F::cast_from(0.16449340668482264365e-1_f64) * t23601 * t23602 * t1014 * t1625 * t25492 + F::cast_from(0.54831135561607547883e-2_f64) * t23633 * t100204 * t25554 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t25712 * t23478 * t25516 + F::new(4.0) * t3186 * t28601 * t4673 + F::cast_from(0.18277045187202515961e-2_f64) * t100215 - t89224 + t11046 * t28596 * t18093 - F::cast_from(0.18277045187202515961e-2_f64) * t82717 - F::new(2.0) * t14608 * t25558 + t1058 * t1945 * t17959 * t1060;
    t100225
}
