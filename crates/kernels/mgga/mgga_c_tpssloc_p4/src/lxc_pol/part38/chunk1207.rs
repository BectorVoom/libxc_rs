//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1207/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1207<F: Float>(t1675: F, t3331: F, t1695: F, t3377: F, t11297: F, t11350: F, t11361: F, t11365: F, t14958: F, t15048: F, t15165: F, t15168: F, t15172: F, t15179: F, t15182: F, t15185: F, t15204: F, t3334: F, t3357: F, t3376: F, t3401: F, t436: F, t4840: F, t4862: F) -> F {
    let t15207 = t1675 * t3331;
    let t15210 = t1695 * t3377;
    let t15213 = F::cast_from(0.64327917994770140268e2_f64) * t3357 * t15165 + F::cast_from(0.32163958997385070134e2_f64) * t3357 * t15168 + F::cast_from(0.2069040516770936012e4_f64) * t11350 * t15172 - F::cast_from(0.23392894490538584828e1_f64) * t11297 * t4840 + F::cast_from(0.34631718211362927518e2_f64) * t11361 * t4862 - F::cast_from(0.23392894490538584828e1_f64) * t3376 * t15179 - F::cast_from(0.11696447245269292414e1_f64) * t3376 * t15182 - F::cast_from(0.10389515463408878255e3_f64) * t11365 * t15185 - F::new(0.310907e-1) * t15204 * t436 + t14958 - F::new(2.0) * t15207 * t3334 + F::cast_from(0.35089341735807877242e1_f64) * t3401 * t15210 + t15048;
    t15213
}
