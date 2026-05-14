//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 725/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk725<F: Float>(t23685: F, t884: F, t6784: F, t2780: F, t6785: F, t225: F, t23592: F, t2771: F, t23661: F, t3188: F, t1945: F, t3120: F, t1060: F, t23571: F, t383: F, t23384: F, t6787: F) -> (F, F, F, F, F, F, F) {
    let t23686 = t23685 * t884;
    let t23687 = t6784 * t23686;
    let t23692 = t6785 * t2780;
    let t23693 = t6784 * t23692;
    let t23696 = t23592 * t225;
    let t23697 = t6785 * t2771;
    let t23698 = t23696 * t23697;
    let t23701 = t23661 * t3188;
    let t23704 = t1945 * t3120;
    let t23705 = t23704 * t1060;
    let t23707 = t383 * t23571;
    let t23712 = t23384 * t6787;
    (t23687, t23693, t23698, t23701, t23705, t23707, t23712)
}
