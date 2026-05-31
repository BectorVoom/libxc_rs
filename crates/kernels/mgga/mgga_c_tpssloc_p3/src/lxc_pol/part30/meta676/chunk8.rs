//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2115/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2115<F: Float>(t2235: F, t5392: F, t16558: F, t17635: F, t17686: F, t17691: F, t1860: F, t1864: F, t1865: F, t22502: F, t22505: F, t26021: F, t26024: F, t26025: F, t26028: F, t26044: F, t26048: F, t27949: F, t27950: F, t27953: F, t27957: F, t5398: F, t6486: F, t6500: F, t6509: F, t67: F, t7428: F, t7441: F, t7445: F, t7446: F, t83791: F, t83796: F, t83803: F) -> F {
    let t96646 = t2235 * t5392;
    let t96649 = -t7428 * t26048 / F::cast_from(3.0_f64) - t26028 * t7446 / F::cast_from(3.0_f64) - t7428 * t26021 / F::cast_from(3.0_f64) - t7428 * t26025 / F::cast_from(3.0_f64) - t6486 * t27950 / F::cast_from(6.0_f64) - t1860 * (-F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t83791 * t5392 - F::cast_from(5.0_f64) / F::cast_from(108.0_f64) * t83796 * t17686 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t22505 * t17691 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t22502 * t5398 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t22505 * t17635 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6500 * t16558 + t83803) * t67 * t1864 / F::cast_from(6.0_f64) - t1860 * t27949 * t6509 / F::cast_from(6.0_f64) - t6486 * t27953 / F::cast_from(3.0_f64) - t1860 * t26044 * t7445 / F::cast_from(3.0_f64) - t1860 * t7441 * t26024 / F::cast_from(3.0_f64) - t6486 * t27957 / F::cast_from(6.0_f64) + t96646 * t1865 / F::cast_from(3.0_f64);
    t96649
}
