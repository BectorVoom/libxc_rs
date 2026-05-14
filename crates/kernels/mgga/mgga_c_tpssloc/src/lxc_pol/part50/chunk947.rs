//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 947/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk947<F: Float>(t30781: F, t884: F, t23329: F, t1945: F, t6688: F, t6691: F, t1955: F, t6815: F, t3174: F, t8376: F, t968: F, t1920: F, t225: F, t387: F, t6768: F, t345: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30782 = t30781 * t884;
    let t30783 = t23329 * t30782;
    let t30788 = t6688 * t1945;
    let t30789 = t30788 * t6691;
    let t30792 = t1955 * t6815;
    let t30793 = t3174 * t30792;
    let t30796 = t968 * t8376;
    let t30798 = 0.54831135561607547883e-2 * t1920 * t30796;
    let t30800 = t6768 * t225 * t387;
    let t30801 = t345 * t30800;
    (t30782, t30783, t30788, t30789, t30793, t30796, t30798, t30800, t30801)
}
