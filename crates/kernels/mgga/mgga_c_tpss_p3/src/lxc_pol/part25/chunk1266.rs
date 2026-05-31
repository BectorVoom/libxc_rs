//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1266/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1266<F: Float>(t21877: F, t21946: F, t3: F, t1799: F, t4637: F, t1338: F, t20690: F, t4674: F, t5953: F, t117: F, t21907: F, t1668: F, t1670: F, t1851: F, t1853: F, t547: F, t5470: F, t5474: F, t5477: F, t548: F, t6446: F, t6452: F, t6455: F, param_d: F) -> (F, F, F, F, F, F, F, F) {
    let t21947 = t21877 + t21946;
    let t21948 = t3 * t21947;
    let t21958 = param_d * t21947;
    let t21972 = t4637 * t1799;
    let t21975 = t20690 * t1338;
    let t21978 = t5953 * t4674;
    let t21981 = t117 * t21907;
    let t21984 = F::cast_from(12.0_f64) * t1668 * t6452 + F::cast_from(6.0_f64) * t1668 * t6455 + F::cast_from(6.0_f64) * t1670 * t6446 + F::cast_from(6.0_f64) * t1851 * t5474 + F::cast_from(3.0_f64) * t1851 * t5477 + F::cast_from(3.0_f64) * t1853 * t5470 + t21958 * t548 + F::cast_from(6.0_f64) * t21972 * t547 + F::cast_from(12.0_f64) * t21975 * t547 + F::cast_from(6.0_f64) * t21978 * t547 + F::cast_from(3.0_f64) * t21981 * t547;
    (t21947, t21948, t21958, t21972, t21975, t21978, t21981, t21984)
}
