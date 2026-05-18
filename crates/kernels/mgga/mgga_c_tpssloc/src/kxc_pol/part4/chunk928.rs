//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 928/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk928<F: Float>(t2570: F, t67: F, t792: F, t12984: F, t686: F, t776: F, t131: F, t9558: F, t205: F, t1489: F, t9541: F, t4126: F, t782: F) -> (F, F, F, F) {
    let t12997 = t2570 * t67;
    let t12998 = t792 * t12997;
    let t13000 = t686 * t12984 * t776;
    let t13002 = F::new(0.49999999999999999998e-2) * t12998 * t13000;
    let t13004 = t9558 * t131;
    let t13005 = t205 * t13004;
    let t13010 = t9541 * t1489;
    let t13012 = t782 * t4126;
    (t13002, t13005, t13010, t13012)
}
