//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1366/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1366<F: Float>(t105727: F, t106671: F, t106677: F, t106686: F, t106690: F, t106699: F, t106706: F, t106712: F, t1649: F, t1877: F, t1915: F, t20390: F, t22959: F, t23295: F, t25013: F, t2522: F, t28: F, t28448: F, t28764: F, t28778: F, t28789: F, t4314: F, t5966: F, t6670: F, t7541: F, t7656: F, t87975: F, t98054: F) -> F {
    let t106716 = F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t7541 * t28778 + F::cast_from(9.0_f64) * t22959 * t106671 + t1877 * t105727 * t28 / F::cast_from(2.0_f64) + F::cast_from(9.0_f64) * t25013 * t106677 + F::cast_from(9.0_f64) * t4314 * t7541 * t28764 + F::cast_from(3.0_f64) * t1877 * t87975 * t28789 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t6670 * t106686 - F::cast_from(9.0_f64) * t25013 * t106690 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t98054 * t7656 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t7541 * t5966 + F::cast_from(3.0_f64) * t1877 * t23295 * t106699 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t28448 * t1649 - F::cast_from(9.0_f64) * t22959 * t106706 + t1877 * t1915 * t20390 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t6670 * t106712;
    t106716
}
