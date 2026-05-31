//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2701/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2701<F: Float>(t16468: F, t225: F, t16458: F, t12023: F, t12027: F, t12033: F, t12237: F, t12444: F, t1386: F, t16022: F, t16437: F, t16453: F, t16460: F, t1834: F, t1843: F, t3752: F, t3758: F, t3882: F, t3889: F, t39910: F, t5318: F, t5321: F, t5326: F, t54738: F, t562: F, t568: F) -> F {
    let t55134 = t16468 * t225;
    let t55150 = t16458 * t225;
    let t55155 = t12237 * t1834 * t568 + F::cast_from(3.0_f64) * t3752 * t5318 * t568 + t54738 * t562 * t568 - F::cast_from(6.0_f64) * t12023 * t5321 + F::cast_from(6.0_f64) * t12027 * t5321 + F::cast_from(6.0_f64) * t12033 * t5326 + F::cast_from(12.0_f64) * t12444 * t5326 - F::cast_from(3.0_f64) * t1386 * t55134 - F::cast_from(3.0_f64) * t1386 * t55150 + F::cast_from(6.0_f64) * t16022 * t3889 - F::cast_from(3.0_f64) * t16437 * t3882 + F::cast_from(12.0_f64) * t16453 * t3758 + F::cast_from(6.0_f64) * t16460 * t3889 - t1843 * t39910;
    t55155
}
