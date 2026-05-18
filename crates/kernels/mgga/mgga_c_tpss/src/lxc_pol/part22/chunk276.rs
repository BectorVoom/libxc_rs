//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 276/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk276<F: Float>(t833: F, t859: F, t839: F, t850: F, t855: F, t863: F) -> (F, F, F) {
    let t879 = F::new(0.516475e0) * t833;
    let t882 = F::new(0.104195e0) * t859;
    let t884 = F::new(0.3529725e1) * t850 - t879 - F::new(0.516475e0) * t839 + F::new(0.6311625e0) * t855 - t882 - F::new(0.104195e0) * t863;
    (t879, t882, t884)
}
