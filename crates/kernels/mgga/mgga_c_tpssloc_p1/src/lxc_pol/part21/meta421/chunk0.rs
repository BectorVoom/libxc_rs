//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1940/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1940<F: Float>(t1751: F, t3493: F, t1246: F, t3507: F, t3625: F, t1932: F, t475: F, t1755: F, t1720: F, t3030: F, t3609: F) -> (F, F, F, F, F, F, F) {
    let t15015 = t1751 * t3493;
    let t15016 = t15015 * t1246;
    let t15018 = t1751 * t3507;
    let t15019 = t15018 * t3625;
    let t15022 = t1932 * t3493 * t475;
    let t15023 = t1755 * t15022;
    let t15026 = t1720 * t3030;
    let t15027 = t15026 * t3609;
    (t15016, t15018, t15019, t15022, t15023, t15026, t15027)
}
