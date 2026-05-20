//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2081;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta633<F: Float>(t19456: F, t6535: F, t22561: F, t4028: F, t193: F, t6829: F, t1530: F, t2379: F, t22960: F, t57893: F, t2745: F, t25373: F) -> (F, F, F, F, F, F, F, F) {
        let (t86700, t86702, t86703, t86706, t86707, t86710, t86713, t86714) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2081::<F>(t19456, t6535, t22561, t4028, t193, t6829, t1530, t2379, t22960, t57893, t2745, t25373);
    (t86700, t86702, t86703, t86706, t86707, t86710, t86713, t86714)
}
