//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta712 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2310;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2311;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta712<F: Float>(t16606: F, t17120: F, t1877: F, t40764: F, t40766: F, t4255: F, t4303: F, t4314: F, t46292: F, t67176: F, t67178: F, t67180: F, t67183: F, t67186: F, t67191: F, t152: F, t20825: F, t607: F, t41284: F, t46302: F, t20742: F, t67: F, t758: F, t58047: F, t58052: F, t58057: F, t40794: F) -> (F, F, F, F, F, F, F, F, F) {
        let t67195 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2310::<F>(t16606, t17120, t1877, t40764, t40766, t4255, t4303, t4314, t46292, t67176, t67178, t67180, t67183, t67186, t67191);
        let (t67204, t67206, t67207, t67210, t67211, t67212, t67214, t67215) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2311::<F>(t152, t20825, t607, t41284, t46302, t20742, t67, t758, t58047, t58052, t58057, t40794);
    (t67195, t67204, t67206, t67207, t67210, t67211, t67212, t67214, t67215)
}
