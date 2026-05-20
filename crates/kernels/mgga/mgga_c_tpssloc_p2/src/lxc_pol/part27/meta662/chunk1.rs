//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2322/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2322<F: Float>(t16405: F, t22833: F, t16387: F, t26309: F, t16275: F, t16271: F, t1336: F, t22759: F, t5252: F, t836: F, t26308: F, t3777: F) -> (F, F, F, F, F, F) {
    let t91103 = t22833 * t16405;
    let t91105 = t26309 * t16387;
    let t91107 = t22833 * t16275;
    let t91109 = t22833 * t16271;
    let t91113 = t1336 * t22759 * t836 * t5252;
    let t91114 = F::new(7.0) / F::new(576.0) * t91113;
    let t91116 = t3777 * t26308 * t5252;
    (t91103, t91105, t91107, t91109, t91114, t91116)
}
