//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1624;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1625;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta441<F: Float>(t23204: F, t6555: F, t23164: F, t6572: F, t6562: F, t2742: F, t6571: F, t6553: F, t1880: F, t2553: F, t6554: F, t6552: F, t212: F, t252: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23205, t23206, t23207, t23208, t23209, t23218, t23219, t23220, t23222, t23223, t23224) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1624::<F>(t23204, t6555, t23164, t6572, t6562, t2742, t6571, t6553, t1880, t2553, t6554, t6552);
        let t23228 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1625::<F>(t212, t252);
    (t23205, t23206, t23207, t23208, t23209, t23218, t23219, t23220, t23222, t23223, t23224, t23228)
}
