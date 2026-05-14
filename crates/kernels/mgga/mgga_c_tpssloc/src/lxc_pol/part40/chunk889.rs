//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 889/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk889<F: Float>(t12945: F, t707: F, t1484: F, t212: F, t9523: F, t2586: F, t2570: F, t67: F, t792: F, t686: F, t776: F, t131: F, t9558: F, t205: F, t1489: F, t9541: F) -> (F, F, F, F, F) {
    let t12946 = t707 * t12945;
    let t12984 = t212 * t1484;
    let t12985 = t9523 * t12984;
    let t12986 = t2586 * t12985;
    let t12997 = t2570 * t67;
    let t12998 = t792 * t12997;
    let t13000 = t686 * t12984 * t776;
    let t13002 = 0.49999999999999999998e-2 * t12998 * t13000;
    let t13004 = t9558 * t131;
    let t13005 = t205 * t13004;
    let t13010 = t9541 * t1489;
    (t12946, t12986, t13002, t13005, t13010)
}
