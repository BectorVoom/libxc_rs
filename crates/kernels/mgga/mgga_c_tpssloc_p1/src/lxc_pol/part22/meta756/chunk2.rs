//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2541/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2541<F: Float>(t21780: F, t3287: F, t1102: F, t3270: F, t21785: F, t43880: F, t18754: F, t4756: F, t14808: F, t5999: F, t18730: F, t4748: F) -> (F, F, F, F, F, F) {
    let t71445 = t3287 * t21780;
    let t71446 = t71445 * t1102;
    let t71448 = t3270 * t21780;
    let t71449 = t71448 * t1102;
    let t71452 = t43880 * t21785 * t1102;
    let t71454 = t18754 * t4756;
    let t71456 = t14808 * t5999;
    let t71458 = t4748 * t18730;
    (t71446, t71449, t71452, t71454, t71456, t71458)
}
