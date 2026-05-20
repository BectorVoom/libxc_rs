//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2044;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta606<F: Float>(t82122: F, t214: F, t2710: F, t23258: F, t6547: F, t794: F, t852: F, t6562: F, t6572: F, t23219: F, t23265: F, t23030: F, t23208: F) -> (F, F, F, F, F, F, F, F) {
        let (t82123, t82124, t82131, t82133, t82135, t82143, t82145, t82147) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2044::<F>(t82122, t214, t2710, t23258, t6547, t794, t852, t6562, t6572, t23219, t23265, t23030, t23208);
    (t82123, t82124, t82131, t82133, t82135, t82143, t82145, t82147)
}
