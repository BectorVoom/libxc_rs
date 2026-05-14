//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1303/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1303<F: Float>(t1270: F, t1760: F, t509: F, t69575: F, t69631: F, t69703: F, t69763: F, t19577: F, t6277: F, t6245: F, t65135: F, t6246: F, t21253: F, t5758: F, t19604: F, t19631: F) -> (F, F, F, F, F, F) {
    let t69768 = t1760 * t509 * (t69575 + t69631 + t69703 + t69763) * t1270;
    let t69770 = 2.0 * t19577 * t6277;
    let t69773 = 6.0 * t1760 * t65135 * t6245;
    let t69775 = 6.0 * t19577 * t6246;
    let t69776 = t21253 * t5758;
    let t69779 = 6.0 * t1760 * t19631 * t19604;
    (t69768, t69770, t69773, t69775, t69776, t69779)
}
