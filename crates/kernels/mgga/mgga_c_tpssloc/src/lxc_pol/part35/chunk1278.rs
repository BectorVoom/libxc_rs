//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1278/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1278<F: Float>(t104907: F, t104968: F, t106836: F, t106855: F, t2110: F, t27332: F, t27961: F, t27972: F, t27976: F, t27982: F, t29475: F, t29478: F, t29481: F, t7432: F, t7435: F, t7975: F, t7978: F, t85501: F, t96120: F) -> (F,) {
    let t109025 = 5.0 / 2.0 * t104968 * t7432 + 5.0 * t27332 * t27972 + 5.0 / 2.0 * t27332 * t27976 + t106855 * t2110 / 3.0 + t27982 * t7975 + t27982 * t7978 + t7435 * t29475 + 2.0 * t7435 * t29478 + t7435 * t29481 - 15.0 * t96120 * t27961 + 35.0 * t85501 * t106836 - 5.0 * t104907 * t7432;
    (t109025,)
}
