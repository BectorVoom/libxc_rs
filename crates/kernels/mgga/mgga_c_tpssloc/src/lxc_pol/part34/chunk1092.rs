//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1092/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1092<F: Float>(t101226: F, t105773: F, t106624: F, t106627: F, t106651: F, t106655: F, t106677: F, t106690: F, t106699: F, t1649: F, t1877: F, t2057: F, t2068: F, t24191: F, t24344: F, t2522: F, t26563: F, t26744: F, t28764: F, t28774: F, t28778: F, t28792: F, t28795: F, t29106: F, t4314: F, t7656: F, t7845: F, t84766: F) -> (F,) {
    let t108616 = 9.0 * t26563 * t106677 + 3.0 * t1877 * t24344 * t106699 + 9.0 * t4314 * t7845 * t28764 - 3.0 / 2.0 * t1877 * t101226 * t7656 + 9.0 / 2.0 * t2522 * t2057 * t106651 + 9.0 / 2.0 * t2522 * t7845 * t28778 - 9.0 / 2.0 * t24191 * t106624 - 3.0 * t1877 * t26744 * t28792 - 9.0 * t26563 * t106690 + 3.0 * t105773 * t2068 - 3.0 * t1877 * t84766 * t106655 + 3.0 / 2.0 * t2522 * t2057 * t106627 - 3.0 / 2.0 * t1877 * t26744 * t28795 + 3.0 / 2.0 * t1877 * t29106 * t1649 + 9.0 * t2522 * t7845 * t28774;
    (t108616,)
}
