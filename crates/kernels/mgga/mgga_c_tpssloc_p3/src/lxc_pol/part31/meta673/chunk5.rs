//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2029/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2029<F: Float>(t1338: F, t29286: F, t2085: F, t6387: F, t1336: F, t1352: F, t16047: F, t16060: F, t19744: F, t19815: F, t27097: F, t27103: F, t29339: F, t29345: F, t3777: F, t5234: F, t5250: F, t5287: F, t5334: F, t5344: F, t6388: F, t7209: F, t7932: F, t84577: F, t91078: F, t91081: F, t93792: F, t93794: F, t97488: F, t97491: F, t97494: F) -> F {
    let t102798 = t1338 * t29286;
    let t102801 = t2085 * t6387;
    let t102822 = F::new(2.0) * t1336 * t84577 * t6388 - F::new(2.0) * t16060 * t7932 + t93792 - F::new(2.0) * t5234 * t27103 - t1336 * t102798 * t1352 - t5344 * t102801 * t1352 + t93794 - F::cast_from(0.10417915756705434098e0_f64) * t91078 + F::new(2.0) * t3777 * t29339 + F::cast_from(0.6579736267392905746e-1_f64) * t91081 + F::cast_from(0.3289868133696452873e-1_f64) * t97488 + F::cast_from(0.6579736267392905746e-1_f64) * t97491 + F::cast_from(0.16449340668482264365e-1_f64) * t97494 - F::new(2.0) * t1336 * t27097 * t5287 - t3777 * t29345 - F::new(6.0) * t16047 * t102801 * t19744 + F::new(6.0) * t5334 * t102801 * t5250 - t19815 * t7209;
    t102822
}
