//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1231/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1231<F: Float>(t16160: F, t16161: F, t16163: F, t16173: F, t225: F, t1345: F, t68: F, t1799: F, t1995: F, t3734: F, t1365: F, t5187: F) -> (F, F, F, F) {
    let t16176 = (t16160 + t16161 + t16163 + t16173) * t225;
    let t16186 = t1345 * t68;
    let t16191 = t1995 * t1799;
    let t16192 = t16191 * t3734;
    let t16195 = t1365 * t5187;
    (t16176, t16186, t16192, t16195)
}
