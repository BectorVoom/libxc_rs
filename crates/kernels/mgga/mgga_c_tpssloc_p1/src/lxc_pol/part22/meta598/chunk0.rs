//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2120/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2120<F: Float>(t3082: F, t4622: F, t41666: F, t43398: F, t10471: F, t47840: F, t10479: F, t10216: F, t13797: F, t3067: F, t353: F, t373: F, t383: F) -> (F, F, F, F, F, F) {
    let t48430 = t4622 * t3082;
    let t48431 = t48430 / F::cast_from(864.0_f64);
    let t48496 = t43398 * t41666;
    let t48569 = t47840 * t10471;
    let t48570 = t48569 * t10479;
    let t48585 = t13797 * t10216;
    let t48607 = t353 * t383 * t3067 * t373;
    (t48431, t48496, t48569, t48570, t48585, t48607)
}
