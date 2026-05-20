//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2430/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2430<F: Float>(t13965: F, t3114: F, t14202: F, t3117: F, t10423: F, t13995: F, t10413: F, t10422: F, t14221: F, t10949: F, t14025: F, t10195: F, t10408: F, t10433: F, t10965: F, t13991: F, t14215: F, t14511: F, t1616: F, t3070: F, t42541: F, t42565: F, t42570: F, t42586: F, t42861: F, t4596: F, t4636: F, t47679: F, t973: F) -> F {
    let t49690 = t3114 * t13965;
    let t49691 = t49690 / F::new(4608.0);
    let t49692 = t3117 * t14202;
    let t49693 = t49692 / F::new(6912.0);
    let t49697 = t13995 * t10423;
    let t49702 = t10413 * t10422 * t14221;
    let t49716 = t10949 * t14025;
    let t49718 = -t42586 / F::new(2304.0) - t49691 - t49693 + F::new(35.0) / F::new(972.0) * t973 * t42861 * t47679 + t49697 / F::new(1152.0) + t10965 * t4636 / F::new(1536.0) - t49702 / F::new(1152.0) + F::new(5.0) / F::new(4608.0) * t3070 * t10408 * t1616 * t10195 + t42541 * t14215 / F::new(384.0) - t14511 * t10433 / F::new(1024.0) + t42565 * t13991 / F::new(32.0) - t42570 * t4596 / F::new(48.0) + t49716 / F::new(384.0);
    t49718
}
