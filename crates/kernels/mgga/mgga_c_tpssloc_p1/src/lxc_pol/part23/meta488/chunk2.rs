//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1496/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1496<F: Float>(t54428: F, t193: F, t20416: F, t3918: F, t3924: F, t39490: F, t39496: F, t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39529: F, t39539: F, t39549: F, t39563: F, t5122: F, t79921: F) -> (F, F) {
    let t79925 = F::cast_from(144.0_f64) * t54428;
    let t79926 = F::cast_from(18.0_f64) * t193 * t3924 * t79921 + F::cast_from(12.0_f64) * t20416 * t3918 * t5122 - t39490 - t39496 + t39499 + t39502 - t39505 - t39508 + t39518 - t39521 - t39529 + t39539 + t39549 + t39563 + t79925;
    (t79925, t79926)
}
