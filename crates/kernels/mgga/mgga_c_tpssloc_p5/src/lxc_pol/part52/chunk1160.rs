//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1160/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1160<F: Float>(t28: F, t6665: F, t1081: F, t1877: F, t2522: F, t30753: F, t30757: F, t30770: F, t6670: F, t6841: F, t6848: F, t8366: F, t8370: F) -> F {
    let t30974 = t28 * t6665;
    let t30982 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8366 * t6841 + t1877 * t30753 * t28 / F::cast_from(2.0_f64) - t1877 * t30757 * t6848 / F::cast_from(2.0_f64) + t1877 * t8366 * t1081 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8370 * t6841 - t1877 * t6670 * t30974 + t1877 * t30770 * t6848 - t1877 * t8370 * t1081 / F::cast_from(2.0_f64);
    t30982
}
