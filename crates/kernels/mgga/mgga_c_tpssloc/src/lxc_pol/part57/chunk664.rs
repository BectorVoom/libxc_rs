//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 664/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk664<F: Float>(t381: F, t7577: F, t23384: F, t7554: F, t7607: F, t225: F, t23665: F, t7611: F, t1625: F, t362: F, t6743: F, t7614: F, t968: F, t1920: F, t7604: F, t4640: F, t6754: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25442 = t7577 * t381;
    let t25450 = t23384 * t7554;
    let t25465 = t23384 * t7607;
    let t25470 = t7577 * t225;
    let t25508 = t23665 * t7611;
    let t25516 = t362 * t1625;
    let t25523 = t7577 * t6743;
    let t25529 = t968 * t7614;
    let t25530 = t1920 * t25529;
    let t25563 = t23384 * t7604;
    let t25577 = t4640 * t6754;
    (t25442, t25450, t25465, t25470, t25508, t25516, t25523, t25530, t25563, t25577)
}
