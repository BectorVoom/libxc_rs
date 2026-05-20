//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta559 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1831;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta559<F: Float>(t1307: F, t19577: F, t1530: F, t2379: F, t22960: F, t57893: F, t2745: F, t25373: F, t25: F, t40772: F, t2749: F, t1408: F, t2752: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t86685, t86706, t86707, t86710, t86713, t86714, t86717, t86718, t86721) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1831::<F>(t1307, t19577, t1530, t2379, t22960, t57893, t2745, t25373, t25, t40772, t2749, t1408, t2752);
    (t86685, t86706, t86707, t86710, t86713, t86714, t86717, t86718, t86721)
}
