//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2352/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2352<F: Float>(t55921: F, t7245: F, t12571: F, t27331: F, t2240: F, t29473: F, t33: F, t2110: F, t26055: F, t26070: F, t26073: F, t26076: F, t26090: F, t27308: F, t27311: F, t27341: F, t6492: F, t7435: F, t7975: F, t7978: F, t96535: F) -> F {
    let t104953 = t55921 * t7245;
    let t104958 = t12571 * t27331;
    let t104968 = t2240 * t33 * t29473;
    let t104971 = F::new(2.0) / F::new(3.0) * t26070 * t7978 + F::new(2.0) / F::new(3.0) * t26073 * t7978 + F::new(2.0) / F::new(3.0) * t26076 * t7978 + F::new(2.0) / F::new(3.0) * t7435 * t27308 + F::new(2.0) / F::new(3.0) * t7435 * t27311 + F::new(5.0) / F::new(6.0) * t104953 * t6492 + t96535 * t2110 / F::new(3.0) + F::new(5.0) / F::new(3.0) * t104958 * t6492 + F::new(2.0) / F::new(3.0) * t26055 * t7975 + F::new(5.0) / F::new(3.0) * t27341 * t26090 + F::new(2.0) / F::new(3.0) * t26055 * t7978 + F::new(5.0) / F::new(6.0) * t104968 * t6492;
    t104971
}
