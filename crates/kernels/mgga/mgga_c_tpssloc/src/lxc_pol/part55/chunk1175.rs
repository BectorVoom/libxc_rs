//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1175/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1175<F: Float>(t22986: F, t23270: F, t30633: F, t98960: F, t112899: F, t25038: F, t25040: F, t1888: F, t32862: F, t82159: F, t112667: F, t112673: F) -> (F, F, F, F, F) {
    let t118488 = F::cast_from(0.6579736267392905746e-1_f64) * t22986 * t23270 * t30633 * t98960;
    let t118491 = F::cast_from(0.9869604401089358619e-1_f64) * t25038 * t112899 * t25040;
    let t118498 = F::cast_from(0.3289868133696452873e-1_f64) * t1888 * t82159 * t32862;
    let t118499 = F::cast_from(0.38381794893125283518e-1_f64) * t112667;
    let t118500 = F::cast_from(0.38381794893125283518e-1_f64) * t112673;
    (t118488, t118491, t118498, t118499, t118500)
}
