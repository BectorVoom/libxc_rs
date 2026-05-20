//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2584/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2584<F: Float>(t15800: F, t225: F, t15808: F, t14731: F, t15419: F, t3447: F, t12606: F, t3450: F, t1714: F, t44583: F, t3451: F, t458: F) -> (F, F, F, F, F, F, F) {
    let t51928 = t15800 * t225;
    let t51937 = t15808 * t225;
    let t51948 = t3447 * t15419 * t14731;
    let t51961 = t3450 * t12606;
    let t51968 = t44583 * t1714;
    let t51970 = t3447 * t51968 * t3451;
    let t51975 = t458 * t1714;
    (t51928, t51937, t51948, t51961, t51968, t51970, t51975)
}
