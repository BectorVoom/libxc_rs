//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2154/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2154<F: Float>(t53096: F, t11647: F, t1731: F, t3577: F, t44951: F, t4949: F, t3242: F, t3448: F, t11718: F, t52835: F, t11147: F, t15394: F) -> (F, F, F, F, F, F) {
    let t53097 = t53096 / F::new(216.0);
    let t53099 = t1731 * t11647;
    let t53161 = t3577 * t44951 * t4949;
    let t53162 = t53161 / F::new(6912.0);
    let t53187 = t3448 * t3242;
    let t53238 = t52835 * t11718;
    let t53249 = t15394 * t11147;
    (t53097, t53099, t53162, t53187, t53238, t53249)
}
