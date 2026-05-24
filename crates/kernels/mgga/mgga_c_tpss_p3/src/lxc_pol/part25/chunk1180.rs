//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1180/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1180<F: Float>(t1675: F, t18646: F, t5784: F, t7682: F, t5483: F, t5791: F, t5506: F, t5790: F, t7690: F) -> (F, F, F, F, F, F) {
    let t18648 = F::new(88.0) / F::new(27.0) * t1675 * t18646;
    let t18649 = t7682 * t5784;
    let t18652 = t5483 * t5791;
    let t18660 = t5790 * t5506;
    let t18661 = t1675 * t18660;
    let t18666 = t7690 * t5784;
    (t18648, t18649, t18652, t18660, t18661, t18666)
}
