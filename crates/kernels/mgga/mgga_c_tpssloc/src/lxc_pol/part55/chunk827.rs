//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 827/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk827<F: Float>(t10143: F, t1914: F, t134: F, t221: F, t3034: F, t371: F, t2752: F, t28: F) -> (F, F, F, F, F) {
    let t23295 = t1914 * t10143;
    let t23383 = t221 * t134;
    let t23508 = 1.0 / t3034 / t371;
    let t23598 = 1.0 / t3034;
    let t23788 = t2752 * t28;
    (t23295, t23383, t23508, t23598, t23788)
}
