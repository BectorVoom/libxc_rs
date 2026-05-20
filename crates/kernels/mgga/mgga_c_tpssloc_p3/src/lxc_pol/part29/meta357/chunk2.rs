//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1449/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1449<F: Float>(t131: F, t9558: F, t205: F, t221: F, t2379: F, t4128: F, t1489: F, t9541: F, t4126: F, t782: F, t4130: F, t12971: F, t210: F, t214: F) -> (F, F, F, F, F) {
    let t13004 = t9558 * t131;
    let t13005 = t205 * t13004;
    let t13007 = t221 * t4128 * t2379;
    let t13010 = t9541 * t1489;
    let t13012 = t782 * t4126;
    let t13014 = F::cast_from(0.23333333333333333332e-1_f64) * t13012 * t4130;
    let t13017 = t210 * t214 * t12971;
    (t13005, t13007, t13010, t13014, t13017)
}
