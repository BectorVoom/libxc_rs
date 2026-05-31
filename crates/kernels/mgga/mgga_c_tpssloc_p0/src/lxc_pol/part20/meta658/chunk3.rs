//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2443/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2443<F: Float>(t10405: F, t10410: F, t10415: F, t10863: F, t10904: F, t10937: F, t13541: F, t13982: F, t13995: F, t14130: F, t14143: F, t14147: F, t14228: F, t3048: F, t3070: F, t3071: F, t4585: F, t49929: F, t49934: F, t49940: F, t49945: F, t49957: F, t49959: F) -> F {
    let t49961 = t49929 * t10405 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t13995 * t10410 - t49934 * t10415 / F::cast_from(1536.0_f64) - t10904 * t13982 / F::cast_from(96.0_f64) + t49940 / F::cast_from(768.0_f64) + t10937 * t14130 / F::cast_from(144.0_f64) - t49945 / F::cast_from(1152.0_f64) + t10863 * t4585 / F::cast_from(72.0_f64) + t3048 * t14143 / F::cast_from(72.0_f64) + t3048 * t14147 / F::cast_from(144.0_f64) - t3070 * t3071 * t13541 * t14228 / F::cast_from(384.0_f64) + t49957 / F::cast_from(768.0_f64) - t49959 / F::cast_from(1536.0_f64);
    t49961
}
