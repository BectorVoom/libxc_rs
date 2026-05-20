//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2536/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2536<F: Float>(t13969: F, t13976: F, t3130: F, t1041: F, t14183: F, t10471: F, t47840: F, t10479: F, t10908: F, t4641: F, t10216: F, t13797: F) -> (F, F, F, F, F, F) {
    let t48564 = t3130 * t13969 * t13976;
    let t48567 = t1041 * t13969 * t14183;
    let t48569 = t47840 * t10471;
    let t48570 = t48569 * t10479;
    let t48574 = t4641 * t10908;
    let t48585 = t13797 * t10216;
    (t48564, t48567, t48569, t48570, t48574, t48585)
}
