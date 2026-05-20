//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta168 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk826;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk827;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta168<F: Float>(t28: F, t1081: F, t3231: F, t3672: F, t517: F, t157: F, t3671: F, zeta_threshold: F, t182: F, t118: F, t521: F) -> (F, F, F, F) {
        let (t3673, t3681) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk826::<F>(t28, t1081, t3231, t3672, t517, t157, t3671, zeta_threshold);
        let (t3683, t3684) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk827::<F>(t182, t3681, t118, t521);
    (t3673, t3681, t3683, t3684)
}
