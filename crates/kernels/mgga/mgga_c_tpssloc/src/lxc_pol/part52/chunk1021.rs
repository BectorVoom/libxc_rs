//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1021/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1021<F: Float>(t7359: F, t7999: F, t1186: F, t8077: F, t1222: F, t8043: F, t6729: F, t8027: F, t2140: F, t4965: F, t1202: F, t8048: F, t8049: F, t5017: F, t7337: F, t1207: F) -> (F, F, F, F, F, F, F, F) {
    let t27572 = t7999 * t7359;
    let t27574 = t1186 * t8077;
    let t27578 = t8043 * t1222;
    let t27580 = t8027 * t6729;
    let t27586 = t4965 * t2140;
    let t27589 = t1202 * t8048;
    let t27592 = t8049 * t1222;
    let t27598 = t7337 * t5017;
    let t27599 = t1207 * t27598;
    (t27572, t27574, t27578, t27580, t27586, t27589, t27592, t27599)
}
