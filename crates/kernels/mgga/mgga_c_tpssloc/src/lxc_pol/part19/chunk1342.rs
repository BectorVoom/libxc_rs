//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1342/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1342<F: Float>(t374: F, t485: F, t486: F, t9697: F, t1090: F, t3493: F, t11786: F, t3490: F, t11154: F, t11784: F, t1227: F, t248: F, t11814: F, t3572: F, t11825: F, t3523: F) -> (F, F, F, F, F, F) {
    let t45250 = 7.0 / 31104.0 * t485 * t374 * t9697 * t486;
    let t45251 = t1090 * t3493;
    let t45256 = t3490 * t11786;
    let t45260 = t1227 * t248 * t11784 * t11154;
    let t45262 = t11814 * t3572;
    let t45264 = t11825 * t3523;
    (t45250, t45251, t45256, t45260, t45262, t45264)
}
