//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 520/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk520<F: Float>(t1089: F, t415: F, t61: F, t1239: F, t496: F, t68: F) -> (F, F, F, F) {
    let t3584 = F::new(1.0) / t415 / t1089;
    let t3585 = t61 * t3584;
    let t3597 = F::new(1.0) / t1239 / t496;
    let t3598 = t68 * t3597;
    (t3584, t3585, t3597, t3598)
}
