//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2026/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2026<F: Float>(t22829: F, t80958: F, t2229: F, t583: F, t60: F, t1995: F, t22816: F, t22818: F, t22765: F, t3858: F, t22764: F, t3777: F) -> (F, F, F, F, F) {
    let t80959 = t80958 * t22829;
    let t80967 = F::new(1.0) / t60 / t2229 / t583;
    let t80970 = t80967 * t1995 * t22816 * t22818;
    let t80971 = F::cast_from(0.43737152435318756759e-3_f64) * t80970;
    let t80989 = t22765 * t3858;
    let t80991 = t3777 * t22764;
    (t80959, t80967, t80971, t80989, t80991)
}
