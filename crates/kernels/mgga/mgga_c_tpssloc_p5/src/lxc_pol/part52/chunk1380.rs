//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1380/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1380<F: Float>(t24932: F, t7468: F, t27888: F, t26003: F, t7266: F, t120005: F, t120008: F, t120019: F, t120020: F, t120022: F, t120027: F, t120029: F, t120040: F, t120044: F, t123062: F, t672: F) -> F {
    let t123138 = t24932 * t7468;
    let t123140 = t27888 * t7468;
    let t123142 = t7266 * t26003;
    let t123151 = -F::new(2.0) * t123062 * t672 - t120005 - t120008 - t120019 - F::new(2.0) * t120020 - F::new(2.0) * t120022 - F::new(2.0) * t120027 - F::new(2.0) * t120029 - F::new(2.0) * t120040 + t120044 - F::new(2.0) * t123138 - F::new(2.0) * t123140 - F::new(2.0) * t123142;
    t123151
}
