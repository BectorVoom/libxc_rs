//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1228/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1228<F: Float>(t22430: F, t580: F, t111: F, t20292: F, t172: F, t20742: F, t763: F, t21066: F, t870: F, t2752: F, t20767: F, t751: F) -> (F, F, F, F, F, F) {
    let t67000 = t22430 * t580;
    let t67001 = t20292 * t111;
    let t67099 = t20742 * t172 * t763;
    let t67112 = t21066 * t870;
    let t67154 = t21066 * t2752;
    let t67159 = t20767 * t751;
    (t67000, t67001, t67099, t67112, t67154, t67159)
}
