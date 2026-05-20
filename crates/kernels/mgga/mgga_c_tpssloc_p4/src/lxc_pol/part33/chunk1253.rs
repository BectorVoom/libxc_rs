//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1253/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1253<F: Float>(t13965: F, t6755: F, t7554: F, t82632: F, t7566: F, t23631: F, t61066: F, t974: F, t23511: F, t7577: F, t7611: F, t82716: F) -> (F, F, F, F, F, F) {
    let t88645 = t6755 * t13965;
    let t88731 = t82632 * t7554;
    let t88882 = t82632 * t7566;
    let t89033 = t23631 * t974 * t61066;
    let t89044 = t7577 * t23511;
    let t89310 = t82716 * t7611;
    (t88645, t88731, t88882, t89033, t89044, t89310)
}
