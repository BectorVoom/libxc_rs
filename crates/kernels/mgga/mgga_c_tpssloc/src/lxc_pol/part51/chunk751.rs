//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 751/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk751<F: Float>(t349: F, t7593: F, t1634: F, t1955: F, t3174: F, t1539: F, t6785: F, t6784: F, t1599: F, t1949: F, t1629: F, t6800: F, t6799: F, t1625: F, t1948: F, t345: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7594 = t349 * t7593;
    let t7599 = t1955 * t1634;
    let t7600 = t3174 * t7599;
    let t7603 = t6785 * t1539;
    let t7604 = t6784 * t7603;
    let t7607 = t1599 * t1949;
    let t7610 = t1629 * t6800;
    let t7611 = t6799 * t7610;
    let t7614 = t1948 * t1625;
    let t7615 = t345 * t7614;
    (t7594, t7600, t7603, t7604, t7607, t7610, t7611, t7614, t7615)
}
