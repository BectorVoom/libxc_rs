//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 915/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk915<F: Float>(t24432: F, t24995: F, t90065: F, t31776: F, t91669: F, t2320: F, t8595: F, t31300: F, t83886: F, t114335: F, t22574: F, t112547: F, t115721: F, t115725: F, t115727: F, t115728: F, t115732: F, t115738: F, t115743: F, t1266: F, t1393: F, t2096: F, t23958: F, t24028: F, t31246: F, t31700: F, t31722: F, t7218: F, t8450: F) -> (F,) {
    let t115748 = 6.0 * t24995 * t24432 * t90065;
    let t115750 = 4.0 * t91669 * t31776;
    let t115752 = 2.0 * t2320 * t8595;
    let t115754 = 6.0 * t83886 * t31300;
    let t115757 = 6.0 * t22574 * t24432 * t114335;
    let t115758 = t112547 * t2096 - 2.0 * t1266 * t31700 + 2.0 * t1393 * t31722 + 6.0 * t23958 * t8450 - 2.0 * t24028 * t8450 + 2.0 * t31246 * t7218 + t115721 - t115725 - t115727 - t115728 - t115732 - t115738 - t115743 - t115748 + t115750 - t115752 - t115754 - t115757;
    (t115758,)
}
