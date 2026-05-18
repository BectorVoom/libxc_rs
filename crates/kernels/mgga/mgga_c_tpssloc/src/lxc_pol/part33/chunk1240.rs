//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1240/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1240<F: Float>(t10336: F, t1920: F, t1949: F, t135: F, t23631: F, t6688: F, t23509: F, t25651: F, t1016: F, t3034: F, t1930: F, t6741: F) -> (F, F, F, F) {
    let t82799 = F::new(0.30461741978670859935e-2) * t1920 * t10336 * t1949;
    let t82822 = t23631 * t135 * t6688;
    let t82895 = t23509 * t25651;
    let t82985 = F::new(1.0) / t3034 / t1016;
    let t82986 = t1930 * t82985;
    let t82987 = t82986 * t6741;
    (t82799, t82822, t82895, t82987)
}
