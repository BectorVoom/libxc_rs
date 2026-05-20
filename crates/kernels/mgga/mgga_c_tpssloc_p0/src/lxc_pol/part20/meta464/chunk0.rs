//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1929/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1929<F: Float>(t15540: F, t4582: F, t12648: F, t4987: F, t13969: F, t4983: F, t3515: F, t486: F, t5011: F) -> (F, F, F, F, F, F) {
    let t15541 = t4582 * t15540;
    let t15544 = t4987 * t12648;
    let t15545 = t4582 * t15544;
    let t15548 = t13969 * t4983;
    let t15550 = t3515 * t15548 / F::new(2304.0);
    let t15553 = t486 * t5011;
    (t15541, t15544, t15545, t15548, t15550, t15553)
}
