//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1208/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1208<F: Float>(t1060: F, t4677: F, t381: F, t4649: F, t1022: F, t1932: F, t360: F) -> (F, F, F, F) {
    let t4678 = t4677 * t1060;
    let t4680 = t381 * t4649;
    let t4681 = t4680 * t1060;
    let t4684 = t1932 * t1022 * t360;
    (t4678, t4680, t4681, t4684)
}
