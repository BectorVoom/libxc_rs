//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 771/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk771<F: Float>(t210: F, t214: F, t5527: F, t5544: F, t2562: F, t2569: F, t2571: F, t2590: F, t4124: F, t4135: F, t787: F, t252: F) -> (F, F, F, F) {
    let t5550 = t210 * t214 * t5527;
    let t5555 = t210 * t214 * t5544;
    let t5558 = t2562 + F::cast_from(0.77777777777777777775e-2_f64) * t4124 + t2569 + F::cast_from(0.49999999999999999998e-2_f64) * t2571 * t5550 + F::cast_from(0.16666666666666666666e-2_f64) * t4135 - F::cast_from(0.16666666666666666666e-2_f64) * t787 * t5555 - t2590;
    let t5559 = t5558 * t252;
    (t5550, t5555, t5558, t5559)
}
