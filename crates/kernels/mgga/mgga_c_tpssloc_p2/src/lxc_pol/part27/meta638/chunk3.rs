//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2156/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2156<F: Float>(t23097: F, t232: F, t46644: F, t815: F, t25135: F, t838: F, t2693: F, t7503: F, t25132: F, t81882: F, t6604: F, t81968: F) -> (F, F, F, F, F) {
    let t87399 = t23097 * t815 * t46644 * t232;
    let t87401 = t25135 * t838;
    let t87402 = F::new(7.0) / F::new(1152.0) * t87401;
    let t87403 = t7503 * t2693;
    let t87405 = t81882 * t25132;
    let t87407 = t81968 * t6604;
    (t87399, t87402, t87403, t87405, t87407)
}
