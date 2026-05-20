//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1217/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1217<F: Float>(t102798: F, t107413: F, t107417: F, t107431: F, t107435: F, t107439: F, t1336: F, t1814: F, t1825: F, t20490: F, t20495: F, t20554: F, t20595: F, t2089: F, t24127: F, t27097: F, t29327: F, t29339: F, t29349: F, t5234: F, t6388: F, t6415: F, t6420: F, t7208: F, t84627: F, t91078: F, t91081: F, t93798: F, t97494: F) -> F {
    let t107987 = -F::cast_from(0.49348022005446793095e-1_f64) * t107413 - F::new(3.0) * t1336 * t27097 * t6420 + F::cast_from(0.29608813203268075857e0_f64) * t107417 + t20595 * t2089 - t1336 * t7208 * t20554 - F::new(6.0) * t5234 * t29349 + F::new(6.0) * t5234 * t29339 + F::new(6.0) * t1336 * t93798 * t6388 - F::new(3.0) * t1336 * t102798 * t1825 - F::new(6.0) * t1336 * t84627 * t20490 + F::new(6.0) * t1336 * t24127 * t20495 - F::cast_from(0.15626873635058151147e0_f64) * t91078 + F::cast_from(0.9869604401089358619e-1_f64) * t91081 + F::cast_from(0.49348022005446793095e-1_f64) * t97494 + F::new(3.0) * t1814 * t29327 - F::cast_from(0.39478417604357434476e0_f64) * t107431 - F::new(3.0) * t1336 * t27097 * t6415 - F::cast_from(0.9869604401089358619e-1_f64) * t107435 + F::cast_from(0.9869604401089358619e-1_f64) * t107439;
    t107987
}
