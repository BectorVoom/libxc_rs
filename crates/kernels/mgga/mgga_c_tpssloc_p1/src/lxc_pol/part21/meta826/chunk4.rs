//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2916/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2916<F: Float>(t17349: F, t2888: F, t13520: F, t14422: F, t10740: F, t10747: F, t10765: F, t10825: F, t14263: F, t14337: F, t14450: F, t14460: F, t17350: F, t17443: F, t17446: F, t17451: F, t17454: F, t17493: F, t17538: F, t17541: F, t17544: F, t17548: F, t17551: F, t17555: F, t2861: F, t2886: F, t41984: F, t42128: F, t42149: F, t4454: F, t4476: F, t49096: F, t49411: F, t60360: F, t931: F, t932: F) -> (F, F) {
    let t60775 = t17349 * t2888;
    let t60787 = F::cast_from(12.0_f64) * t13520 * t14422;
    let t60806 = -F::cast_from(8.0_f64) * t10740 * t17538 - F::cast_from(0.38596750796862084162e3_f64) * t41984 * t17541 - F::cast_from(4.0_f64) * t10740 * t17544 + F::cast_from(0.64327917994770140268e2_f64) * t10765 * t17548 - F::cast_from(4.0_f64) * t2861 * t17350 * t931 + F::cast_from(0.64327917994770140268e2_f64) * t2886 * t60775 * t931 + F::cast_from(0.12865583598954028054e3_f64) * t10765 * t17551 + F::cast_from(0.4138081033541872024e4_f64) * t42149 * t17555 - F::cast_from(4.0_f64) * t2861 * t60360 * t932 - t60787 - F::cast_from(0.46785788981077169656e1_f64) * t49096 * t4454 + F::cast_from(0.69263436422725855034e2_f64) * t49411 * t4476 - F::cast_from(0.46785788981077169656e1_f64) * t14263 * t14450 + F::cast_from(0.69263436422725855034e2_f64) * t14337 * t14460 + F::cast_from(0.70178683471615754484e1_f64) * t10825 * t17443 - F::cast_from(0.46785788981077169656e1_f64) * t10747 * t17446 - F::cast_from(0.2077903092681775651e3_f64) * t42128 * t17451 - F::cast_from(0.23392894490538584828e1_f64) * t10747 * t17454 + F::cast_from(0.34631718211362927518e2_f64) * t10825 * t17493;
    (t60787, t60806)
}
