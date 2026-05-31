//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2333/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2333<F: Float>(t24574: F, t27392: F, t1170: F, t2121: F, t27766: F, t1238: F, t15794: F, t1716: F, t24567: F, t24568: F, t24582: F, t24630: F, t24639: F, t24877: F, t24893: F, t27406: F, t27415: F, t3598: F, t3630: F, t4945: F, t5055: F, t5060: F, t7283: F, t7351: F, t8087: F, t86473: F, t86494: F) -> F {
    let t95863 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27392;
    let t95866 = F::cast_from(0.54831135561607547884e-2_f64) * t2121 * t1170 * t27766;
    let t95876 = -F::cast_from(6.0_f64) * t7351 * t15794 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t24568 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t24630 + F::cast_from(4.0_f64) * t4945 * t24582 - F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t24639 + F::cast_from(4.0_f64) * t24893 * t5060 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t24567 * t27415 + F::cast_from(0.12184696791468343974e-2_f64) * t86473 + t95863 + t95866 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t86494 + F::cast_from(2.0_f64) * t1238 * t3598 * t8087 * t3630 + F::cast_from(2.0_f64) * t5055 * t24877;
    t95876
}
