//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2971/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2971<F: Float>(t1041: F, t13969: F, t17687: F, t14085: F, t4571: F, t13765: F, t13995: F, t18086: F, t3069: F, t10403: F, t10413: F, t10891: F, t14041: F, t14130: F, t14218: F, t17718: F, t2776: F, t3041: F, t3070: F, t3071: F, t3073: F, t3121: F, t3132: F, t42483: F, t4582: F, t4650: F, t47779: F, t48611: F, t49658: F, t49661: F, t49666: F, t5685: F, t5867: F, t61855: F) -> F {
    let t61923 = t1041 * t13969 * t17687;
    let t61929 = t14085 * t4571;
    let t61940 = t13995 * t13765;
    let t61950 = t18086 * t3069;
    let t61965 = -F::new(5.0) / F::new(1728.0) * t61923 + F::new(5.0) / F::new(384.0) * t1041 * t4582 * t47779 * t61855 + t61929 / F::new(1728.0) + t10891 * t17718 / F::new(288.0) - F::new(4.0) / F::new(243.0) * t49658 - t49661 / F::new(243.0) + t49666 / F::new(3456.0) + t42483 * t48611 * t14218 * t4650 / F::new(768.0) + t61940 / F::new(1728.0) - t13995 * t14130 / F::new(1152.0) + t13995 * t14041 / F::new(2304.0) - t3070 * t3071 * t5867 * t2776 / F::new(2304.0) + t61950 * t3073 / F::new(2304.0) + t3070 * t3071 * t5685 * t3121 / F::new(4608.0) + t10403 * t3071 * t5685 * t3132 / F::new(2304.0) - t10413 * t3071 * t5685 * t3041 / F::new(4608.0);
    t61965
}
