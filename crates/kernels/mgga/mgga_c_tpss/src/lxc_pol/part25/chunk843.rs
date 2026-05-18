//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 843/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk843<F: Float>(t5: F, t1675: F, t1792: F, t5785: F, t5793: F, t6073: F, t6077: F, t6080: F, t6304: F, t117: F) -> (F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t6308 = piecewise3::<f64>(t8, F::new(0.0), t6073 * t1792 / F::new(3.0) - F::new(5.0) / F::new(3.0) * t5785 * t6077 - F::new(2.0) / F::new(3.0) * t6080 * t1792 - t5793 + t1675 * t6304 / F::new(3.0));
    let t6309 = t6308 * t117;
    (t6308, t6309)
}
