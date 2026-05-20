//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2665/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2665<F: Float>(t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t53783: F, t53788: F, t53797: F, t73958: F, t73959: F, t73960: F, t73961: F, t73962: F, t73968: F, t73969: F, t74013: F) -> F {
    let t74466 = -t73958 - t73959 + t53783 + t53788 - t73960 - t73961 - t73962 - t39249 - t39256 - t73968 + t53797 - t73969 - t39261 - t39266 - t39304 + t74013;
    t74466
}
