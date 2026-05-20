//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2124/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2124<F: Float>(t4021: F, t72: F, t7431: F, t1864: F, t5389: F, t12571: F, t1410: F, t27971: F, t645: F, t1437: F, t7445: F, t27975: F) -> (F, F, F, F, F, F) {
    let t96422 = t72 * t7431 * t4021;
    let t96425 = t1864 * t5389;
    let t96443 = t12571 * t1410;
    let t96458 = t72 * t27971 * t645;
    let t96461 = t7445 * t1437;
    let t96466 = t72 * t27975 * t645;
    (t96422, t96425, t96443, t96458, t96461, t96466)
}
