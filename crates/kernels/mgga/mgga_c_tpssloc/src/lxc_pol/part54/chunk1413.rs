//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1413/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1413<F: Float>(t113941: F, t115306: F, t115308: F, t115318: F, t115331: F, t120201: F, t120209: F, t120213: F, t122121: F, t122127: F, t122131: F, t122133: F, t16030: F, t8637: F) -> F {
    let t122137 = -t115306 + F::cast_from(0.41123351671205660912e-2_f64) * t122121 + F::cast_from(0.41123351671205660912e-2_f64) * t115308 + F::cast_from(0.16449340668482264365e-1_f64) * t122127 + F::cast_from(0.16449340668482264365e-1_f64) * t122131 + t120201 - t113941 + F::cast_from(0.19190897446562641759e-1_f64) * t122133 - F::cast_from(0.82246703342411321824e-2_f64) * t115318 - t16030 * t8637 - t115331 + t120209 + t120213;
    t122137
}
