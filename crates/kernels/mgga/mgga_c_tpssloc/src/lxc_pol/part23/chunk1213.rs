//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1213/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1213<F: Float>(t75852: F, t75862: F, t75875: F, t75891: F, t75934: F, t75947: F, t76543: F, t76556: F, t41666: F, t75836: F, t123: F, t41664: F) -> (F, F, F) {
    let t76559 = t75852 + t75862 + t75875 + t75891 + t75934 + t75947 + t76543 + t76556;
    let t76572 = t41666 * t75836;
    let t76574 = t123 * t41664 * t76572;
    (t76559, t76572, t76574)
}
