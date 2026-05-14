//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 242/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk242<F: Float>(t942: F, t950: F, t951: F, t959: F, t338: F, t615: F, t134: F, t340: F, t344: F) -> (F, F, F, F, F) {
    let t961 = t942 * t950 * t951;
    let t963 = 0.5848223622634646207e0 * t959 * t961;
    let t964 = t615 * t338;
    let t967 = t134 * t340;
    let t968 = t967 * t344;
    (t961, t963, t964, t967, t968)
}
