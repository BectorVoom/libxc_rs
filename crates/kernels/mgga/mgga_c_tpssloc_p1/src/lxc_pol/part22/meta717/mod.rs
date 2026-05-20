//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta717 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2325;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta717<F: Float>(t20816: F, t2427: F, t46369: F, t46371: F, t46376: F, t58984: F, t41259: F, t46433: F, t39593: F, t41254: F, t41258: F, t41262: F, t46336: F, t67472: F, t67475: F, t67478: F, t67480: F, t67482: F) -> (F, F, F, F, F, F, F, F) {
        let (t67484, t67485, t67486, t67487, t67488, t67489, t67490, t67491) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2325::<F>(t20816, t2427, t46369, t46371, t46376, t58984, t41259, t46433, t39593, t41254, t41258, t41262, t46336, t67472, t67475, t67478, t67480, t67482);
    (t67484, t67485, t67486, t67487, t67488, t67489, t67490, t67491)
}
