//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 746/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk746<F: Float>(t24049: F, t24050: F, t24058: F, t24060: F, t24061: F, t26272: F, t26295: F, t28085: F, t28089: F, t28091: F, t28093: F, t28095: F, t28097: F, t28102: F, t28104: F, t29274: F) -> (F,) {
    let t29285 = 0.80745512188280781706e-3 * t26272 + t28085 / 384.0 - t24049 + t24050 + 0.56521858531796547194e-2 * t26295 + t28089 / 768.0 - t28091 / 768.0 + 5.0 / 192.0 * t28093 - t28095 / 192.0 - t28097 / 96.0 + 0.48447307312968469024e-2 * t28102 + t24058 + t24060 + t24061 + t28104 / 96.0;
    let t29286 = t29274 + t29285;
    (t29286,)
}
