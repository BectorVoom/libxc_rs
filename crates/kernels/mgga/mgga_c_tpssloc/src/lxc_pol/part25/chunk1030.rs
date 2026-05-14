//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1030/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1030<F: Float>(t23080: F, t81963: F, t200: F, t23075: F, t598: F, t213: F, t1894: F, t236: F, t9458: F, t23034: F, t6546: F, t23037: F, t131: F, t845: F, t1878: F, t209: F) -> (F, F, F, F, F) {
    let t81964 = t81963 * t23080;
    let t81968 = t598 / t23075 / t200;
    let t81969 = t81968 * t213;
    let t81972 = t81969 * t1894 * t236 * t9458;
    let t81979 = t6546 * t23034;
    let t81980 = t81979 * t23037;
    let t81982 = t845 * t131;
    let t81984 = t1878 * t81982 * t209;
    (t81964, t81972, t81979, t81980, t81984)
}
