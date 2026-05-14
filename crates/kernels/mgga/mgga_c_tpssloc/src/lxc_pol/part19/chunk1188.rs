//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1188/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1188<F: Float>(t41961: F, t136: F, t2826: F, t41705: F, t10304: F, t41693: F, t41715: F, t908: F, t41644: F, t41937: F, t41940: F, t41943: F, t41945: F, t41948: F, t41951: F, t41954: F, t41957: F, t41959: F) -> (F, F, F, F, F) {
    let t41962 = 0.13490888888888888889e1 * t41961;
    let t41964 = t136 * t2826 * t41705;
    let t41967 = t136 * t10304 * t41693;
    let t41970 = t136 * t908 * t41715;
    let t41973 = t136 * t908 * t41644;
    let t41975 = -0.485484375e1 * t41937 - 0.3883875e1 * t41940 + 0.6189328125e-1 * t41943 + 0.247573125e0 * t41945 - 0.51785e1 * t41948 + 0.3300975e0 * t41951 + 0.11651625e2 * t41954 - 0.247573125e0 * t41957 + t41959 + t41962 - 0.11038e0 * t41964 - 0.22076e0 * t41967 - 0.298026e1 * t41970 + 0.66228e0 * t41973;
    (t41964, t41967, t41970, t41973, t41975)
}
