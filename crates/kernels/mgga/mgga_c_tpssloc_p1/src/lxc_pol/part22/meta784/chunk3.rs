//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2694/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2694<F: Float>(t1338: F, t20601: F, t1336: F, t1352: F, t16040: F, t16060: F, t16132: F, t1825: F, t19668: F, t19732: F, t20473: F, t20625: F, t20643: F, t3777: F, t5234: F, t5334: F, t5348: F, t5351: F, t57659: F, t6378: F, t6415: F, t6448: F, t6451: F) -> F {
    let t75124 = t1338 * t20601;
    let t75150 = -t1336 * t1352 * t75124 - F::cast_from(3.0_f64) * t1336 * t16132 * t6415 - F::cast_from(3.0_f64) * t1336 * t1825 * t57659 - F::cast_from(3.0_f64) * t1336 * t19732 * t5348 + F::cast_from(6.0_f64) * t16040 * t20473 * t5334 + F::cast_from(6.0_f64) * t16060 * t6448 - F::cast_from(6.0_f64) * t16060 * t6451 + F::cast_from(6.0_f64) * t19668 * t5234 + F::cast_from(6.0_f64) * t20625 * t3777 - t20643 * t3777 + F::cast_from(3.0_f64) * t5351 * t6378;
    t75150
}
