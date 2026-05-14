//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 872/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk872<F: Float>(t1599: F, t23588: F, t23384: F, t7554: F, t1065: F, t7624: F, t3174: F, t7614: F, t986: F, t6805: F, t7607: F, t1949: F, t4542: F, t225: F, t7577: F, t6786: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25447 = t1599 * t23588;
    let t25450 = t23384 * t7554;
    let t25452 = t7624 * t1065;
    let t25453 = t3174 * t25452;
    let t25456 = t986 * t7614;
    let t25459 = t1599 * t6805;
    let t25465 = t23384 * t7607;
    let t25467 = t4542 * t1949;
    let t25470 = t7577 * t225;
    let t25471 = t25470 * t6786;
    (t25447, t25450, t25452, t25453, t25456, t25459, t25465, t25467, t25470, t25471)
}
