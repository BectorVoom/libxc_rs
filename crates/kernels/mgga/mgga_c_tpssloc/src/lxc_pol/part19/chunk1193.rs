//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1193/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1193<F: Float>(t41654: F, t41961: F, t41937: F, t41940: F, t41943: F, t41945: F, t41948: F, t41951: F, t41954: F, t41957: F, t41964: F, t41967: F, t41970: F, t41973: F, t42046: F, t42061: F, t42077: F, t893: F, t913: F) -> (F,) {
    let t42086 = 0.31003950617283950618e1 * t41654;
    let t42087 = 0.13388493827160493828e1 * t41961;
    let t42092 = -0.3560484375e1 * t41937 - 0.28483875e1 * t41940 + 0.1151859375e0 * t41943 + 0.46074375e0 * t41945 - 0.379785e1 * t41948 + 0.614325e0 * t41951 + 0.85451625e1 * t41954 - 0.46074375e0 * t41957 + t42086 + t42087 - 0.10954222222222222222e0 * t41964 - 0.21908444444444444444e0 * t41967 - 0.295764e1 * t41970 + 0.65725333333333333332e0 * t41973;
    let t42097 = 1.0 * t893 * (t42046 + t42061 + t42077 + t42092) * t913;
    (t42097,)
}
