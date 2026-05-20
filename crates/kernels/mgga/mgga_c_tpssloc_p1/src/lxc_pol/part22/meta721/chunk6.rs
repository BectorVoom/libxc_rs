//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2350/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2350<F: Float>(t68077: F, t68102: F, t68124: F, t68141: F, t225: F, t21008: F, t9573: F, t13228: F, t1495: F, t1510: F, t16662: F, t16836: F, t16851: F, t16928: F, t210: F, t237: F, t249: F, t2571: F, t2643: F, t41130: F, t41139: F, t41363: F, t4178: F, t46692: F, t47039: F, t47080: F, t47094: F, t47231: F, t47270: F, t58569: F, t59100: F) -> (F, F, F) {
    let t68143 = t68077 + t68102 + t68124 + t68141;
    let t68144 = t68143 * t225;
    let t68148 = t9573 * t21008;
    let t68150 = F::new(3.0) / F::new(16.0) * t2571 * t210 * t1495 * t16662 + t47080 - F::new(595.0) / F::new(10368.0) * t41130 + t41139 - t47094 + F::new(595.0) / F::new(10368.0) * t41363 - t47231 + F::new(7.0) / F::new(4.0) * t59100 + F::new(3.0) / F::new(512.0) * t4178 * t46692 * t13228 * t58569 - t16836 * t16928 / F::new(64.0) + F::new(15.0) / F::new(128.0) * t2643 * t47039 * t1510 * t16851 + t68144 * t237 * t249 / F::new(3072.0) - F::new(7.0) / F::new(16.0) * t68148 - t47270;
    (t68143, t68144, t68150)
}
