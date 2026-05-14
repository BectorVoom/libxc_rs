//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 797/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk797<F: Float>(t5: F, t1675: F, t1680: F, t5483: F, t5487: F, t5489: F, t5492: F, t5503: F, t5507: F, t117: F, t116: F, t1683: F) -> (F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t5511 = piecewise3(t8, 0.0, -t5483 * t1680 / 6.0 + 5.0 / 6.0 * t5487 * t5489 + t5492 * t1680 / 3.0 - t1675 * t5503 / 6.0 - t1675 * t5507 / 6.0);
    let t5512 = t5511 * t117;
    let t5514 = t1683 * t116;
    (t5511, t5512, t5514)
}
