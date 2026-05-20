//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1364/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1364<F: Float>(t105758: F, t23788: F, t20800: F, t28: F, t21066: F, t1649: F, t5527: F, t1484: F, t5966: F, t5544: F, t20778: F, t105773: F, t106618: F, t106621: F, t1877: F, t1915: F, t1969: F, t22959: F, t2522: F, t25358: F, t25372: F, t28448: F, t28771: F, t28774: F, t28792: F, t28795: F, t4314: F, t6670: F, t7541: F, t7649: F, t82312: F, t86736: F) -> F {
    let t106624 = t23788 * t105758;
    let t106627 = t28 * t20800;
    let t106636 = t28 * t21066;
    let t106640 = t1649 * t5527;
    let t106647 = t5966 * t1484;
    let t106651 = t1649 * t5544;
    let t106655 = t28 * t20778;
    let t106667 = F::new(3.0) * t25372 * t106618 - F::new(9.0) / F::new(2.0) * t22959 * t106621 - F::new(9.0) / F::new(2.0) * t22959 * t106624 + F::new(3.0) / F::new(2.0) * t2522 * t1915 * t106627 + F::new(3.0) * t105773 * t1969 + F::new(9.0) * t2522 * t7541 * t28774 - t1877 * t6670 * t106636 / F::new(2.0) + F::new(9.0) * t4314 * t1915 * t106640 + F::new(9.0) / F::new(2.0) * t2522 * t28448 * t7649 + F::new(9.0) / F::new(2.0) * t2522 * t1915 * t106647 + F::new(9.0) / F::new(2.0) * t2522 * t1915 * t106651 - F::new(3.0) * t1877 * t82312 * t106655 - F::new(3.0) / F::new(2.0) * t1877 * t25358 * t28795 - F::new(9.0) * t86736 * t28771 - F::new(3.0) * t1877 * t25358 * t28792;
    t106667
}
