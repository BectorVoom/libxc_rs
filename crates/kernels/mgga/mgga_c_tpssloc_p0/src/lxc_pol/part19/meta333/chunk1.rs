//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1195/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1195<F: Float>(t40722: F, t2523: F, t39400: F, t39408: F, t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t40708: F, t40711: F, t40714: F, t40716: F, t40721: F, t4314: F, t9616: F) -> (F, F) {
    let t40723 = F::cast_from(0.22787578869697033845e-2_f64) * t40722;
    let t40724 = F::new(72.0) * t2523 * t4314 * t9616 - t39400 + t39408 + t39411 + t39463 - t39468 - t39472 - t39476 + t40708 + t40711 - t40714 + t40716 - t40721 - t40723;
    (t40723, t40724)
}
