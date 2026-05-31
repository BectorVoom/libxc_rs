//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2736/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2736<F: Float>(t12021: F, t12030: F, t1375: F, t1378: F, t16030: F, t16413: F, t16437: F, t16439: F, t16453: F, t16471: F, t1807: F, t1843: F, t19648: F, t20051: F, t20060: F, t3758: F, t3882: F, t3888: F, t3889: F, t3911: F, t5215: F, t5321: F, t5326: F, t5354: F, t539: F, t55134: F, t568: F, t57485: F, t57526: F, t57564: F, t57597: F, t57631: F, t57667: F, t57692: F, t57725: F, t57760: F, t6439: F, t6440: F, t6460: F) -> F {
    let t57795 = -F::cast_from(4.0_f64) * t16439 * t5354 + t539 * t57485 * t568 + F::cast_from(4.0_f64) * t5215 * t16471 + F::cast_from(2.0_f64) * t20060 * t3889 - t1375 * t1378 * (t57526 + t57564 + t57597 + t57631 + t57667 + t57692 + t57725 + t57760) + F::cast_from(2.0_f64) * t12030 * t6440 - F::cast_from(2.0_f64) * t55134 * t1843 - F::cast_from(12.0_f64) * t3758 * t20051 - F::cast_from(6.0_f64) * t1375 * t12021 * t6460 * t3888 - F::cast_from(12.0_f64) * t3882 * t20051 + F::cast_from(8.0_f64) * t3758 * t19648 - F::cast_from(6.0_f64) * t1375 * t12021 * t6439 * t3911 - F::cast_from(4.0_f64) * t16030 * t5354 + F::cast_from(2.0_f64) * t1807 * t16413 * t568 + F::cast_from(8.0_f64) * t5321 * t16453 - F::cast_from(2.0_f64) * t5215 * t16437 + F::cast_from(8.0_f64) * t16439 * t5326;
    t57795
}
