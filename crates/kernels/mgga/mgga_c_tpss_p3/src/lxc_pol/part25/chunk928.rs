//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 928/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk928<F: Float>(t125: F, t3664: F, t3671: F, t8313: F, t1385: F, t8130: F, t2383: F, t3689: F, t2143: F, t3622: F, t1369: F, t8176: F) -> (F, F, F, F, F, F) {
    let t10590 = t125 * t3664;
    let t10600 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t8313 * t3671;
    let t10617 = t8130 * t1385;
    let t10620 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t2383 * t3689;
    let t10630 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t2143 * t3622;
    let t10635 = t8176 * t1369;
    (t10590, t10600, t10617, t10620, t10630, t10635)
}
