//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1278/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1278<F: Float>(t909: F, t9709: F, t10310: F, t699: F, t10304: F, t136: F, t41688: F, t2403: F, t2833: F, t2827: F, t10322: F, t10306: F) -> (F, F, F, F, F, F, F) {
    let t41863 = t9709 * t909;
    let t41865 = t699 * t10310;
    let t41868 = t136 * t10304 * t41688;
    let t41870 = t2403 * t2833;
    let t41872 = t2403 * t2827;
    let t41874 = t699 * t10322;
    let t41876 = t699 * t10306;
    (t41863, t41865, t41868, t41870, t41872, t41874, t41876)
}
