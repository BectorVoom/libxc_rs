//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 744/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk744<F: Float>(t3806: F, t866: F, t846: F, t1424: F, t2533: F, t865: F, t2531: F, t2455: F, t2537: F, t3746: F, t3751: F, t3756: F, t3760: F) -> (F, F, F, F, F, F) {
    let t3807 = t3806 * t866;
    let t3809 = F::new(1.0) * t846 * t3807;
    let t3810 = t1424 * t2533;
    let t3811 = t3810 * t865;
    let t3813 = F::cast_from(0.16081979498692535067e2_f64) * t2531 * t3811;
    let t3819 = t2537 + F::cast_from(0.57077777777777777777e-2_f64) * t2455 + F::cast_from(0.57077777777777777777e-2_f64) * t3746 - F::cast_from(0.11415555555555555555e-1_f64) * t3751 + F::cast_from(0.34246666666666666666e-1_f64) * t3756 - F::cast_from(0.17123333333333333333e-1_f64) * t3760;
    (t3807, t3809, t3810, t3811, t3813, t3819)
}
