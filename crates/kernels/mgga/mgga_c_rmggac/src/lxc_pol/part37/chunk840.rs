//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 840/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk840<F: Float>(t77916: F, t4669: F, t558: F, t71949: F, t71940: F, t326: F, t650: F, t9565: F, t25854: F, t77327: F, t27176: F, t77330: F, t3851: F, t71982: F, t8642: F, t71983: F, t8646: F) -> (F, F, F, F, F, F, F, F) {
    let t77917 = 0.44903406381989282115e-1 * t77916;
    let t77919 = t4669 * t71949 * t558;
    let t77920 = 0.11974241701863808564e0 * t77919;
    let t77921 = 0.39914139006212695213e-1 * t71940;
    let t77929 = t326 * t9565 * t650;
    let t77930 = 0.34093327067806677161e-2 * t77929;
    let t77933 = 0.35922725105591425692e0 * t25854 * t77327;
    let t77935 = 0.47896966807455234256e0 * t27176 * t77330;
    let t77937 = t3851 * t71982 * t8642;
    let t77938 = 0.20455996240684006296e-1 * t77937;
    let t77939 = t71983 * t8646;
    (t77917, t77920, t77921, t77930, t77933, t77935, t77938, t77939)
}
