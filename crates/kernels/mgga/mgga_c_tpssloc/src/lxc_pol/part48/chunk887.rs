//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 887/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk887<F: Float>(t31776: F, t91669: F, t2320: F, t8595: F, t31300: F, t83886: F, t114335: F, t22574: F, t24432: F, t191: F, t192: F, t24026: F, t2020: F, t15904: F, t36740: F, t22579: F, t8607: F) -> (F, F, F, F, F, F, F) {
    let t115750 = 4.0 * t91669 * t31776;
    let t115752 = 2.0 * t2320 * t8595;
    let t115754 = 6.0 * t83886 * t31300;
    let t115757 = 6.0 * t22574 * t24432 * t114335;
    let t115765 = t24026 * t191 * t192;
    let t115766 = t115765 * t2020;
    let t115771 = 6.0 * t22574 * t36740 * t15904;
    let t115773 = t8607 * t22579;
    (t115750, t115752, t115754, t115757, t115766, t115771, t115773)
}
