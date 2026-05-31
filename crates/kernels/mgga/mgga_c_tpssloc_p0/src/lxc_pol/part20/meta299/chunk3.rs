//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1519/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1519<F: Float>(t10622: F, t10649: F, t10652: F, t10654: F, t10657: F, t10665: F, t1068: F, t10699: F, t10707: F, t10715: F, t10739: F, t10819: F, t10855: F, t3209: F, t3216: F, t4700: F) -> F {
    let t11103 = -F::cast_from(3.0_f64) * t1068 * t3209 * t3216 * t4700 + t10622 - t10649 + t10652 + t10654 + t10657 - t10665 + t10699 + t10707 + t10715 + t10739 - t10819 - t10855;
    t11103
}
