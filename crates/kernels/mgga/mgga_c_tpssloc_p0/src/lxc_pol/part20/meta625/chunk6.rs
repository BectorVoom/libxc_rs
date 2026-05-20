//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2255/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2255<F: Float>(t13312: F, t9638: F, t41107: F, t4240: F, t13261: F, t2617: F, t812: F, t836: F, t9972: F, t13265: F, t13258: F, t13333: F) -> (F, F, F, F, F) {
    let t46717 = t9638 * t13312;
    let t46733 = t41107 * t4240;
    let t46737 = t2617 * t13261;
    let t46741 = t812 * t9972 * t836;
    let t46742 = t46741 * t13265;
    let t46748 = t13258 * t13333;
    (t46717, t46733, t46737, t46742, t46748)
}
