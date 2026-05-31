//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1373/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1373<F: Float>(t1006: F, t1497: F, t1692: F, t1812: F, t18728: F, t20025: F, t20054: F, t20417: F, t20510: F, t20514: F, t20526: F, t21659: F, t2439: F, t5853: F, t6354: F, t70800: F, t70803: F, t70844: F, t70850: F, t70854: F, t70868: F, t70906: F, t70929: F, t70932: F, t72298: F, t72310: F) -> F {
    let t72561 = -F::cast_from(6.0_f64) * t20417 * t70800 + F::cast_from(6.0_f64) * t18728 * t70854 + F::cast_from(6.0_f64) * t20417 * t70906 + t1692 * t20510 * t1497 + F::cast_from(3.0_f64) * t2439 * t1812 * t70868 - t1692 * t20514 * t20054 - F::cast_from(3.0_f64) * t18728 * t70932 + F::cast_from(3.0_f64) * t20417 * t70929 - t1692 * t5853 * t70850 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2439 * t6354 * t20025 + t1692 * t21659 * t1006 / F::cast_from(2.0_f64) - t72298 + F::cast_from(6.0_f64) * t20417 * t70844 + t20526 * t70803 - t72310;
    t72561
}
