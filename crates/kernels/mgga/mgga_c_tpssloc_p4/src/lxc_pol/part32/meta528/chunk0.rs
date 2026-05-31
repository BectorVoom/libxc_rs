//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1863/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1863<F: Float>(t27363: F, t67: F, t1864: F, t1860: F, t2110: F, t24520: F, t24526: F, t26055: F, t26063: F, t26067: F, t26090: F, t27332: F, t27341: F, t6486: F, t6492: F, t6495: F, t7246: F, t7256: F, t7259: F, t7432: F, t7435: F, t7975: F, t7978: F) -> (F, F, F) {
    let t27364 = t27363 * t67;
    let t27365 = t27364 * t1864;
    let t27368 = t7435 * t7256 / F::cast_from(3.0_f64) + t7435 * t7259 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t27332 * t6492 + t6495 * t7975 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t26090 + t6495 * t7978 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t27341 * t6492 + t26055 * t2110 / F::cast_from(3.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t24520 * t7432 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t24526 * t7432 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t26063 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t7246 * t26067 - t6486 * t7975 / F::cast_from(6.0_f64) - t1860 * t27365 / F::cast_from(6.0_f64);
    (t27364, t27365, t27368)
}
