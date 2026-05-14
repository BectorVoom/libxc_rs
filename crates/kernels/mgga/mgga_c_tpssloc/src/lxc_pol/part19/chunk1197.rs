//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1197/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1197<F: Float>(t41654: F, t41961: F, t41937: F, t41940: F, t41943: F, t41945: F, t41948: F, t41951: F, t41954: F, t41957: F, t41964: F, t41967: F, t41970: F, t41973: F, t2884: F, t302: F) -> (F, F) {
    let t42212 = 0.5356037037037037037e1 * t41654;
    let t42213 = 0.16979925925925925926e1 * t41961;
    let t42218 = -0.6618234375e1 * t41937 - 0.52945875e1 * t41940 + 0.2366859375e0 * t41943 + 0.94674375e0 * t41945 - 0.705945e1 * t41948 + 0.1262325e1 * t41951 + 0.158837625e2 * t41954 - 0.94674375e0 * t41957 + t42212 + t42213 - 0.13892666666666666667e0 * t41964 - 0.27785333333333333334e0 * t41967 - 0.375102e1 * t41970 + 0.83356e0 * t41973;
    let t42224 = t2884 * t2884;
    let t42226 = t302 / t42224;
    (t42218, t42226)
}
