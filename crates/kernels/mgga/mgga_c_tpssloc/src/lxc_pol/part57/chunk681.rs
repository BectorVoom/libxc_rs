//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 681/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk681<F: Float>(t112: F, t7945: F, t19299: F, t33: F, t22505: F, t22510: F, t5392: F, t5398: F, t6500: F, t67: F, t1864: F, t7441: F, t7445: F, t5441: F, t71: F, t1863: F) -> (F, F, F, F, F, F, F) {
    let t27254 = t7945 * t112;
    let t27937 = t19299 * t33;
    let t27948 = 5.0 / 18.0 * t22505 * t5392 + 5.0 / 6.0 * t6500 * t5398 - t22510;
    let t27949 = t27948 * t67;
    let t27950 = t27949 * t1864;
    let t27953 = t7441 * t7445;
    let t27956 = t71 * t5441;
    let t27957 = t1863 * t27956;
    (t27254, t27937, t27948, t27950, t27953, t27956, t27957)
}
