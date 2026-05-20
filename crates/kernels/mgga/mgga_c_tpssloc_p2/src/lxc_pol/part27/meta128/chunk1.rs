//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 753/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk753<F: Float>(t2836: F, t913: F, t893: F, t891: F, t275: F) -> (F, F, F, F, F) {
    let t2837 = t2836 * t913;
    let t2839 = F::new(1.0) * t893 * t2837;
    let t2840 = t891 * t891;
    let t2841 = F::new(1.0) / t2840;
    let t2842 = t275 * t2841;
    (t2837, t2839, t2840, t2841, t2842)
}
