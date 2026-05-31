//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 979/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk979<F: Float>(t26043: F, t67: F, t1864: F, t6509: F, t7441: F, t12571: F, t6489: F, t1860: F, t1865: F, t22544: F, t22549: F, t22551: F, t26009: F, t26013: F, t26016: F, t26021: F, t26025: F, t26028: F, t6486: F, t6492: F, t6506: F, t6510: F, t7428: F, t7442: F, t7446: F) -> F {
    let t26044 = t26043 * t67;
    let t26045 = t26044 * t1864;
    let t26048 = t7441 * t6509;
    let t26051 = t12571 * t6489;
    let t26054 = -F::cast_from(5.0_f64) * t22544 * t26009 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t22549 * t26013 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26016 * t22551 - t6486 * t7446 / F::cast_from(6.0_f64) - t1860 * t26021 / F::cast_from(6.0_f64) - t1860 * t26025 / F::cast_from(6.0_f64) - t26028 * t1865 / F::cast_from(6.0_f64) - t7428 * t6506 / F::cast_from(6.0_f64) - t7428 * t6510 / F::cast_from(6.0_f64) - t6486 * t7442 / F::cast_from(6.0_f64) - t1860 * t26045 / F::cast_from(6.0_f64) - t1860 * t26048 / F::cast_from(6.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t26051 * t6492;
    t26054
}
