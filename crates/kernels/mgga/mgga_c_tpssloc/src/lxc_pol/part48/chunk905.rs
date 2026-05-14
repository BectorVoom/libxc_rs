//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 905/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk905<F: Float>(t115743: F, t115748: F, t115750: F, t115752: F, t115754: F, t115757: F, t115766: F, t115771: F, t115773: F, t115777: F, t115781: F, t116135: F, t116304: F, t12734: F, t2096: F, t2165: F, t2314: F, t23933: F, t24008: F, t24433: F, t32318: F, t7040: F, t7266: F, t7408: F, t8835: F) -> (F,) {
    let t117648 = -6.0 * t116135 * t24433 + t116304 * t2096 - 4.0 * t12734 * t8835 - t2165 * t24008 - 4.0 * t2314 * t32318 - 4.0 * t23933 * t7266 - 2.0 * t7040 * t7408 - t115743 - t115748 + t115750 - t115752 - t115754 - t115757 + t115766 - t115771 - t115773 + t115777 - t115781;
    (t117648,)
}
