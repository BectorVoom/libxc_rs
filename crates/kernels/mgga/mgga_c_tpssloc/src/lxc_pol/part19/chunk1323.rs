//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1323/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1323<F: Float>(t44493: F, t44547: F, t44600: F, t44655: F, t3630: F, t3493: F, t491: F, t11720: F, t1235: F, t10469: F, t1190: F, t11887: F, t42339: F, t466: F, t11715: F, t42341: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44657 = t44493 + t44547 + t44600 + t44655;
    let t44662 = t3630 * t3630;
    let t44668 = t3493 * t3493;
    let t44669 = t491 * t44668;
    let t44673 = t1235 * t11720;
    let t44690 = t1190 * t10469;
    let t44691 = t44690 * t11887;
    let t44696 = t466 * t42339;
    let t44698 = t44696 * t42341 * t11715;
    (t44657, t44662, t44668, t44669, t44673, t44690, t44691, t44696, t44698)
}
