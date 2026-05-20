//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 779/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk779<F: Float>(t2354: F, t9403: F, t9393: F, t103: F, t100: F, t2336: F, t2343: F, t2346: F, t657: F, t660: F, t92: F, t9374: F, t9386: F, t9390: F, t9394: F, t9400: F, t96: F) -> (F, F) {
    let t9404 = t9403 * t2354;
    let t9407 = -t9393;
    let t9408 = t103 * t9407;
    let t9411 = -F::new(440.0) / F::new(27.0) * t9374 * t96 + F::new(200.0) / F::new(9.0) * t2336 * t660 - F::new(50.0) / F::new(9.0) * t657 * t2343 - F::new(25.0) / F::new(3.0) * t657 * t2346 - F::new(10.0) / F::new(27.0) * t92 * t9386 + F::new(10.0) / F::new(3.0) * t92 * t9390 + F::new(5.0) / F::new(3.0) * t92 * t9394 - F::new(10.0) / F::new(27.0) * t100 * t9400 + F::new(10.0) / F::new(3.0) * t100 * t9404 + F::new(5.0) / F::new(3.0) * t100 * t9408;
    (t9407, t9411)
}
