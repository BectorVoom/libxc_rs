//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1353/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1353<F: Float>(t68500: F, t68502: F, t68504: F, t68506: F, t76877: F, t76880: F, t76887: F, t76890: F, t76893: F, t76896: F, t76899: F, t136: F, t76624: F, t908: F) -> (F, F) {
    let t76901 = t76877 / F::new(6.0) - F::new(2.0) * t76880 - F::new(16.0) / F::new(81.0) * t68500 - F::new(4.0) / F::new(9.0) * t68502 - F::new(8.0) / F::new(3.0) * t68504 + F::new(8.0) / F::new(9.0) * t68506 + F::new(14.0) / F::new(81.0) * t76887 + t76890 / F::new(6.0) + F::new(2.0) * t76893 - F::new(8.0) / F::new(9.0) * t76896 + F::new(4.0) / F::new(9.0) * t76899;
    let t76903 = t136 * t908 * t76624;
    (t76901, t76903)
}
