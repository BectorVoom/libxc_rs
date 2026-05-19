//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1170/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1170<F: Float>(t23204: F, t6555: F, t23164: F, t6572: F, t6562: F, t6624: F, t798: F, t1911: F, t2719: F, t10110: F, t2742: F, t6571: F) -> (F, F, F, F, F, F, F) {
    let t23205 = t23204 * t6555;
    let t23206 = t23164 * t23205;
    let t23207 = F::cast_from(0.16449340668482264365e-1_f64) * t23206;
    let t23208 = t23204 * t6572;
    let t23209 = t6562 * t23208;
    let t23211 = t798 * t6624;
    let t23214 = t1911 * t2719;
    let t23215 = t10110 * t23214;
    let t23218 = t6571 * t2742;
    (t23205, t23207, t23208, t23209, t23211, t23215, t23218)
}
