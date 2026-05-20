//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2893/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2893<F: Float>(t14379: F, t49226: F, t2791: F, t5689: F, t2794: F, t4433: F, t2792: F, t2836: F, t5727: F, t10661: F, t17520: F, t2793: F) -> (F, F, F, F, F) {
    let t60354 = F::new(24.0) * t49226 * t14379;
    let t60357 = t5689 * t2791;
    let t60359 = F::new(2.0) * t60357 * t2794;
    let t60360 = t4433 * t4433;
    let t60371 = F::new(2.0) * t2792 * t5727 * t2836;
    let t60374 = F::cast_from(0.96491876992155210402e2_f64) * t10661 * t17520 * t2793;
    (t60354, t60359, t60360, t60371, t60374)
}
