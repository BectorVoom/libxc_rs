//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1267/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1267<F: Float>(t1351: F, t1992: F, t550: F, t6976: F, t7918: F, t115391: F, t120441: F, t120445: F, t120447: F, t120452: F, t120456: F, t120459: F, t120463: F, t120467: F, t122451: F, t122457: F, t122460: F, t122462: F, t1336: F, t1814: F, t31636: F, t31639: F, t5287: F) -> (F,) {
    let t122467 = t1992 * t6976 * t7918 * t1351 * t550;
    let t122470 = 0.49348022005446793095e-1 * t122451 - t1336 * t31636 * t5287 + t120441 - t120445 + t120447 - t120452 - 0.82246703342411321825e-2 * t122457 + 0.41123351671205660912e-2 * t122460 - t120456 + t120459 + t120463 + t120467 + 0.19190897446562641759e-1 * t122462 - 0.82246703342411321825e-2 * t122467 - t115391 + t1814 * t31639;
    (t122470,)
}
