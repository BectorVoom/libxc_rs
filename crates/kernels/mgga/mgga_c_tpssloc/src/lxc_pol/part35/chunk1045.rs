//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1045/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1045<F: Float>(t134: F, t221: F, t3034: F, t371: F, t2752: F, t28: F, t2274: F, t50: F, t7245: F, t9239: F, t2127: F) -> (F, F, F, F, F, F, F) {
    let t23383 = t221 * t134;
    let t23508 = 1.0 / t3034 / t371;
    let t23598 = 1.0 / t3034;
    let t23788 = t2752 * t28;
    let t24498 = t50 * t2274;
    let t24514 = t9239 * t7245;
    let t24574 = t2127 * t23383;
    (t23383, t23508, t23598, t23788, t24498, t24514, t24574)
}
