//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2091/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2091<F: Float>(t24645: F, t7999: F, t2121: F, t3427: F, t8010: F, t24574: F, t27416: F, t27794: F, t27441: F, t85639: F, t27446: F, t1751: F, t225: F, t461: F) -> (F, F, F, F, F, F, F) {
    let t94427 = F::cast_from(0.14621636149762012769e-1_f64) * t7999 * t24645;
    let t94436 = t2121 * t3427 * t8010;
    let t94439 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27416;
    let t94446 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27794;
    let t94451 = F::cast_from(0.18277045187202515961e-2_f64) * t85639 * t27441;
    let t94456 = F::cast_from(0.36554090374405031922e-2_f64) * t85639 * t27446;
    let t94458 = t461 * t1751 * t225;
    (t94427, t94436, t94439, t94446, t94451, t94456, t94458)
}
