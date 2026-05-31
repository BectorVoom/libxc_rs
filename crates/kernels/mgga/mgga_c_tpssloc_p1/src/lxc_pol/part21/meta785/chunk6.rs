//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2725/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2725<F: Float>(t12300: F, t6417: F, t19868: F, t3799: F, t12283: F, t19958: F, t12351: F, t12407: F, t12429: F, t1363: F, t16018: F, t16060: F, t16148: F, t16153: F, t16224: F, t16391: F, t1799: F, t1825: F, t19876: F, t19882: F, t19956: F, t3719: F, t3803: F, t3805: F, t3807: F, t3870: F, t40293: F, t5245: F, t5252: F, t54585: F, t54607: F, t54609: F, t54611: F, t54750: F, t56817: F, t6330: F, t820: F) -> F {
    let t57407 = t12300 * t6417;
    let t57409 = t3799 * t19868;
    let t57437 = t12283 * t19958;
    let t57447 = -F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t1363 * t12351 * t820 * t6330 * t3719 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t57407 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t57409 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t54585 + F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t1363 * t3870 * t820 * t1799 * t16018 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t40293 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t54607 + t3803 * t3805 * t19956 * t12407 / F::cast_from(768.0_f64) - F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t54609 - t19876 * t16391 / F::cast_from(192.0_f64) + t16060 * t5245 * t5252 / F::cast_from(384.0_f64) + F::cast_from(119.0_f64) / F::cast_from(864.0_f64) * t54611 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t54750 + t3803 * t3805 * t56817 * t3807 / F::cast_from(384.0_f64) + t12429 * t19882 / F::cast_from(384.0_f64) - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t57437 - F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t3803 * t16224 * t1825 * t16148 - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t3803 * t16224 * t1825 * t16153;
    t57447
}
