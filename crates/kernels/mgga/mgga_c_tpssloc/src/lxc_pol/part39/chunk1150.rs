//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1150/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1150<F: Float>(t120: F, t5187: F, t1352: F, t3805: F, t3851: F, t5301: F, t1810: F, t210: F, t3734: F, t3856: F, t3793: F, t5248: F, t5249: F, t3802: F, t5234: F, t3788: F, t836: F) -> (F, F, F, F, F, F, F, F) {
    let t16364 = t120 * t5187;
    let t16366 = t3805 * t16364 * t1352;
    let t16370 = t3805 * t5301 * t3851;
    let t16379 = t210 * t1810 * t3734;
    let t16383 = t3805 * t5301 * t3856;
    let t16387 = t5248 * t5249 * t3793;
    let t16391 = t3805 * t5301 * t3793;
    let t16394 = t5234 * t3802;
    let t16397 = t3788 * t836;
    (t16366, t16370, t16379, t16383, t16387, t16391, t16394, t16397)
}
