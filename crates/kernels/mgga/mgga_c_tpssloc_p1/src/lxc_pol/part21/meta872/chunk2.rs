//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3214/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3214<F: Float>(t5091: F, t11947: F, t6270: F, t193: F, t336: F, t3637: F, t3640: F, t4700: F, t64436: F, t64441: F, t65301: F, t65305: F, t65307: F, t65309: F, t65312: F, t65314: F, t65319: F, t65321: F, t65324: F, t65326: F) -> F {
    let t66892 = t5091 * t5091;
    let t66897 = t6270 * t11947;
    let t66901 = -F::new(2.0) * t193 * t336 * t3640 * t66892 + F::new(2.0) * t3637 * t4700 * t66897 + t64436 - t64441 - t65301 + t65305 + t65307 - t65309 + t65312 - t65314 - t65319 - t65321 - t65324 - t65326;
    t66901
}
