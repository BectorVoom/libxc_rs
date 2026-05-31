//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1387/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1387<F: Float>(t1649: F, t5544: F, t20778: F, t28: F, t105773: F, t106618: F, t106621: F, t106624: F, t106627: F, t106636: F, t106640: F, t106647: F, t1877: F, t1915: F, t1969: F, t22959: F, t2522: F, t25358: F, t25372: F, t28448: F, t28771: F, t28774: F, t28792: F, t28795: F, t4314: F, t6670: F, t7541: F, t7649: F, t82312: F, t86736: F) -> F {
    let t106651 = t1649 * t5544;
    let t106655 = t28 * t20778;
    let t106667 = F::cast_from(3.0_f64) * t25372 * t106618 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t22959 * t106621 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t22959 * t106624 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t106627 + F::cast_from(3.0_f64) * t105773 * t1969 + F::cast_from(9.0_f64) * t2522 * t7541 * t28774 - t1877 * t6670 * t106636 / F::cast_from(2.0_f64) + F::cast_from(9.0_f64) * t4314 * t1915 * t106640 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t28448 * t7649 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t106647 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t106651 - F::cast_from(3.0_f64) * t1877 * t82312 * t106655 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t25358 * t28795 - F::cast_from(9.0_f64) * t86736 * t28771 - F::cast_from(3.0_f64) * t1877 * t25358 * t28792;
    t106667
}
