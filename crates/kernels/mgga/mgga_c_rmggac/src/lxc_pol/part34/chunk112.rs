//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 112/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk112<F: Float>(t209: F, t469: F, t476: F, t6: F, t263: F, t9: F, t31: F, t212: F, t222: F, t466: F, t140: F, t198: F, t219: F) -> (F, F, F, F, F, F, F) {
    let t479 = t469 * t6 * t476 * t209;
    let t483 = 1.0 / t9 / t263;
    let t484 = t31 * t483;
    let t487 = 0.64025631606094613569e-1 * t212 * t484 * t222;
    let t488 = t212 * t466;
    let t489 = t198 * t140;
    let t490 = t219 * t6;
    (t479, t483, t484, t487, t488, t489, t490)
}
