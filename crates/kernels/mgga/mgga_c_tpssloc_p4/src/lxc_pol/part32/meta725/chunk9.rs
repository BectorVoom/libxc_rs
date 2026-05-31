//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2338/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2338<F: Float>(t225: F, t29687: F, t104453: F, t1252: F, t1721: F, t1761: F, t2155: F, t254: F, t27396: F, t27406: F, t27549: F, t27742: F, t27761: F, t27767: F, t27775: F, t27779: F, t27786: F, t29532: F, t3593: F, t466: F, t498: F, t5055: F, t65208: F, t7999: F, t94514: F, t94779: F, t95824: F, t95902: F) -> F {
    let t104556 = t29687 * t225;
    let t104564 = -F::cast_from(12.0_f64) * t1721 * t254 * t27786 + F::cast_from(4.0_f64) * t3593 * t29532 + F::cast_from(4.0_f64) * t5055 * t27396 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t27779 - F::cast_from(0.43864908449286038306e-1_f64) * t7999 * t27767 + F::cast_from(4.0_f64) * t5055 * t27761 - F::cast_from(2.0_f64) * t5055 * t27742 - t65208 * t2155 - F::cast_from(0.73108180748810063845e-2_f64) * t27549 * t94514 * t27775 - t94779 - F::cast_from(2.0_f64) * t104556 * t1252 + F::cast_from(0.97477574331746751793e-2_f64) * t95824 - F::cast_from(2.0_f64) * t95902 * t1761 + t466 * t104453 * t498;
    t104564
}
