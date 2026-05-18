//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 974/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk974<F: Float>(t225: F, t24600: F, t1089: F, t1240: F, t3597: F, t1235: F, t7284: F, t1251: F, t2122: F, t1170: F, t7295: F, t2121: F) -> (F, F, F, F, F, F) {
    let t24601 = t24600 * t225;
    let t24602 = t1240 * t1089;
    let t24615 = t225 * t3597;
    let t24633 = t7284 * t1235;
    let t24637 = t1240 * t1251;
    let t24638 = t2122 * t24637;
    let t24645 = t1170 * t7295;
    let t24646 = t2121 * t24645;
    (t24601, t24602, t24615, t24633, t24638, t24646)
}
