//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1279/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1279<F: Float>(t50834: F, t71335: F, t71337: F, t77959: F, t77963: F, t77967: F, t77971: F, t77975: F, t77979: F, t77983: F, t77989: F, t77992: F, t77995: F, t77998: F, t11145: F, t123: F, t77957: F) -> (F, F) {
    let t78000 = 0.44152e0 * t77959 - 0.8585111111111111111e-1 * t77963 - 0.82785e-1 * t77967 + 0.49671e0 * t77971 - 0.99342e0 * t77975 + 0.198684e1 * t77979 + 0.82785e-1 * t77983 + 0.22076e0 * t71335 - 0.132456e1 * t71337 - 0.12524296296296296297e1 * t50834 + 0.72462e1 * t77989 + 0.301925e0 * t77992 - 0.89459259259259259259e0 * t77995 + 0.181155e1 * t77998;
    let t78002 = t123 * t11145 * t77957;
    (t78000, t78002)
}
