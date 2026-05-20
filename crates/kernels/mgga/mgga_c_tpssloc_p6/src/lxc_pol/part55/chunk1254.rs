//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1254/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1254<F: Float>(t120705: F, t22574: F, t8643: F, t26179: F, t8327: F, t31058: F, t7458: F, t19456: F, t4028: F, t12725: F, t55353: F, t8319: F) -> (F, F, F, F, F, F, F) {
    let t120708 = F::new(6.0) * t22574 * t8643 * t120705;
    let t120719 = F::new(2.0) * t26179 * t8327;
    let t120721 = F::new(2.0) * t7458 * t31058;
    let t120728 = F::new(2.0) * t19456 * t8327;
    let t120730 = F::new(2.0) * t4028 * t31058;
    let t120735 = F::new(2.0) * t12725 * t8327;
    let t120786 = F::new(27.0) * t55353 * t8319;
    (t120708, t120719, t120721, t120728, t120730, t120735, t120786)
}
