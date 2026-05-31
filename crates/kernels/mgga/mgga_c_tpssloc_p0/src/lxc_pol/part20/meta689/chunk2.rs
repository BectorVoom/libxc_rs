//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2613/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2613<F: Float>(t1227: F, t13969: F, t15616: F, t11731: F, t11741: F, t11781: F, t45007: F, t45009: F, t45013: F, t5024: F, t53079: F, t53083: F, t53087: F, t53093: F, t53097: F, t53099: F) -> F {
    let t53102 = t1227 * t13969 * t15616;
    let t53106 = t53079 / F::cast_from(10368.0_f64) + t53083 * t11731 / F::cast_from(96.0_f64) - t53087 * t11741 / F::cast_from(576.0_f64) + F::cast_from(5.0_f64) / F::cast_from(972.0_f64) * t5024 * t11781 + t53093 / F::cast_from(256.0_f64) + t53097 + t45007 / F::cast_from(4608.0_f64) + t53099 / F::cast_from(10368.0_f64) - t53102 / F::cast_from(384.0_f64) - t45009 / F::cast_from(2304.0_f64) - t45013 / F::cast_from(6912.0_f64);
    t53106
}
