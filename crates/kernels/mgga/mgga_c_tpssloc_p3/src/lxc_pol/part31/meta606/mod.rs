//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta606 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1851;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta606<F: Float>(t2091: F, t40590: F, t90500: F, t90511: F, t225: F, t27070: F, t27052: F, t90514: F, t90524: F, t90533: F, t90541: F, t90546: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t93319, t93333, t93337, t93338, t93341, t93344, t93350, t93353, t93359, t93361) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1851::<F>(t2091, t40590, t90500, t90511, t225, t27070, t27052, t90514, t90524, t90533, t90541, t90546);
    (t93319, t93333, t93337, t93338, t93341, t93344, t93350, t93353, t93359, t93361)
}
