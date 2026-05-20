//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta610 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1855;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta610<F: Float>(t91154: F, t91158: F, t91161: F, t91170: F, t91214: F, t91225: F, t91281: F, t91283: F, t91286: F, t91290: F, t91300: F, t91303: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93651, t93652, t93653, t93657, t93674, t93682, t93710, t93711, t93712, t93715, t93718, t93720) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1855::<F>(t91154, t91158, t91161, t91170, t91214, t91225, t91281, t91283, t91286, t91290, t91300, t91303);
    (t93651, t93652, t93653, t93657, t93674, t93682, t93710, t93711, t93712, t93715, t93718, t93720)
}
