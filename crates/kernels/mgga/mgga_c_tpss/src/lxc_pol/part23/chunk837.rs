//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 837/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk837<F: Float>(t5: F, t1675: F, t1861: F, t5483: F, t5489: F, t5492: F, t5966: F, t5976: F, t5979: F, t117: F) -> (F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t5983 = piecewise3(t8, 0.0, -t5483 * t1861 / 6.0 + 5.0 / 6.0 * t5966 * t5489 + t5492 * t1861 / 3.0 - t1675 * t5976 / 6.0 - t1675 * t5979 / 6.0);
    let t5984 = t5983 * t117;
    (t5983, t5984)
}
