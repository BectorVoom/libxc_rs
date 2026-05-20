//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1969/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1969<F: Float>(t1251: F, t8087: F, t3598: F, t225: F, t497: F, t5052: F, t462: F, t24574: F, t8006: F, t3242: F, t3961: F, t24601: F) -> (F, F, F, F, F, F, F) {
    let t27760 = t8087 * t1251;
    let t27761 = t3598 * t27760;
    let t27766 = t5052 * t225 * t497;
    let t27767 = t462 * t27766;
    let t27770 = t24574 * t8006;
    let t27774 = t497 * t3242;
    let t27775 = t27774 * t3961;
    let t27776 = t24601 * t27775;
    (t27761, t27766, t27767, t27770, t27774, t27775, t27776)
}
