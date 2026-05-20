//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1960/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1960<F: Float>(t1011: F, t6224: F, t3508: F, t24661: F, t475: F, t24668: F, t2132: F, t28525: F, t1726: F, t2136: F, t24659: F, t27674: F, t27677: F, t27681: F, t27701: F, t6178: F, t6184: F, t6188: F, t6207: F, t7310: F, t7345: F) -> (F, F, F, F, F, F) {
    let t29642 = t6224 * t1011;
    let t29643 = t29642 * t3508;
    let t29644 = t24661 * t29643;
    let t29647 = t29642 * t475;
    let t29648 = t24668 * t29647;
    let t29651 = t2132 * t28525;
    let t29662 = -t7345 * t6207 / F::new(2304.0) - t27677 / F::new(54.0) - F::cast_from(0.16149102437656156342e-2_f64) * t27681 + F::cast_from(0.20186378047070195428e-3_f64) * t27701 + F::cast_from(0.20186378047070195428e-3_f64) * t24659 * t29644 - F::cast_from(0.10093189023535097714e-3_f64) * t24659 * t29648 - F::cast_from(0.10093189023535097714e-3_f64) * t29651 * t2136 + t7310 * t6178 / F::new(216.0) + t27674 * t1726 / F::new(54.0) - t7310 * t6184 / F::new(288.0) - t7310 * t6188 / F::new(144.0);
    (t29643, t29644, t29647, t29648, t29651, t29662)
}
