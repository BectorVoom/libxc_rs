//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta534 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta534<F: Float>(t131: F, t1365: F, t154: F, t21: F, t6896: F, t6898: F, t213: F, t6924: F, t9223: F, t6928: F, t22715: F, t547: F) -> (F, F, F, F, F, F, F) {
        let (t80730, t80741, t80742, t80743, t80766, t80767, t80775) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1748::<F>(t131, t1365, t154, t21, t6896, t6898, t213, t6924, t9223, t6928, t22715, t547);
    (t80730, t80741, t80742, t80743, t80766, t80767, t80775)
}
