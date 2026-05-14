//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1225/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1225<F: Float>(t105731: F, t25927: F, t20947: F, t25891: F, t1649: F, t5660: F, t105762: F, t23788: F, t5664: F, t28248: F, t89992: F, t1530: F, t5966: F, t105727: F, t1877: F, t1915: F, t20390: F, t22959: F, t23295: F, t25013: F, t2522: F, t28: F, t28448: F, t28764: F, t28778: F, t28789: F, t4314: F, t6670: F, t7541: F, t7656: F, t87975: F, t98054: F) -> (F,) {
    let t106671 = t25927 * t105731;
    let t106677 = t25891 * t20947;
    let t106686 = t1649 * t5660;
    let t106690 = t23788 * t105762;
    let t106699 = t1649 * t5664;
    let t106706 = t89992 * t28248;
    let t106712 = t5966 * t1530;
    let t106716 = 9.0 / 2.0 * t2522 * t7541 * t28778 + 9.0 * t22959 * t106671 + t1877 * t105727 * t28 / 2.0 + 9.0 * t25013 * t106677 + 9.0 * t4314 * t7541 * t28764 + 3.0 * t1877 * t87975 * t28789 - 3.0 / 2.0 * t1877 * t6670 * t106686 - 9.0 * t25013 * t106690 - 3.0 / 2.0 * t1877 * t98054 * t7656 + 3.0 / 2.0 * t1877 * t7541 * t5966 + 3.0 * t1877 * t23295 * t106699 + 3.0 / 2.0 * t1877 * t28448 * t1649 - 9.0 * t22959 * t106706 + t1877 * t1915 * t20390 / 2.0 - 3.0 / 2.0 * t1877 * t6670 * t106712;
    (t106716,)
}
