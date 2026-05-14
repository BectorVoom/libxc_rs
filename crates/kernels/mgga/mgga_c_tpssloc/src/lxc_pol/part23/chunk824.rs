//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 824/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk824<F: Float>(t13004: F, t205: F, t1489: F, t9541: F, t4126: F, t782: F, t4134: F, t9546: F, t1496: F, t2528: F, t4199: F, t2663: F, t4211: F, t2535: F, t1471: F, t32: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13005 = t205 * t13004;
    let t13010 = t9541 * t1489;
    let t13012 = t782 * t4126;
    let t13022 = t9546 * t4134;
    let t13087 = t9541 * t1496;
    let t13107 = t4199 * t2528;
    let t13109 = t4211 * t2663;
    let t13113 = t4199 * t2535;
    let t13115 = t32 * t1471;
    (t13005, t13010, t13012, t13022, t13087, t13107, t13109, t13113, t13115)
}
