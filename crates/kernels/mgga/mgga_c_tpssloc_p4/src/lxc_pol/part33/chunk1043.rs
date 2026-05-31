//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1043/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1043<F: Float>(t11310: F, t11365: F, t1148: F, t15126: F, t15136: F, t15207: F, t21827: F, t21901: F, t21907: F, t21939: F, t21942: F, t21947: F, t21952: F, t21956: F, t21958: F, t21960: F, t21963: F, t21975: F, t21990: F, t3357: F, t3401: F, t436: F, t4835: F, t6037: F, t6069: F, t6085: F, t6088: F) -> F {
    let t21991 = -t21901 + F::cast_from(0.17544670867903938621e1_f64) * t4835 * t6085 + F::cast_from(0.51947577317044391276e2_f64) * t15126 * t6088 - F::cast_from(0.10389515463408878255e3_f64) * t11365 * t21907 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t21939 + F::cast_from(0.10254018858216406658e4_f64) * t11310 * t21942 - F::cast_from(0.35089341735807877242e1_f64) * t15136 * t6069 + F::cast_from(0.35089341735807877242e1_f64) * t3401 * t21947 - F::cast_from(6.0_f64) * t15207 * t6037 + F::cast_from(6.0_f64) * t3357 * t21952 - t21956 - t21958 - t21960 + t21963 - F::cast_from(0.19751673498613801407e-1_f64) * t21827 - F::cast_from(0.310907e-1_f64) * t21975 * t436 + t21990;
    t21991
}
