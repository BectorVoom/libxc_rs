//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2138/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2138<F: Float>(t22986: F, t25249: F, t2684: F, t6646: F, t25324: F, t6562: F, t794: F, t23030: F, t25258: F, t13384: F, t2647: F, t22893: F, t23164: F, t25306: F) -> (F, F, F, F, F) {
    let t87150 = t22986 * t6646 * t25249 * t2684;
    let t87153 = t6562 * t794 * t25324;
    let t87154 = F::cast_from(0.82246703342411321824e-2_f64) * t87153;
    let t87155 = t23030 * t25258;
    let t87159 = t22986 * t6646 * t13384 * t2647;
    let t87165 = t23164 * t22893 * t25306;
    (t87150, t87154, t87155, t87159, t87165)
}
