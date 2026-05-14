//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 988/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk988<F: Float>(t5343: F, t72: F, t732: F, t10019: F, t10028: F, t12754: F, t12757: F, t12769: F, t12780: F, t13623: F, t13624: F, t13631: F, t13637: F, t13645: F, t9954: F, t9956: F, t9972: F, t9980: F) -> (F, F) {
    let t13806 = t5343 * t72;
    let t13807 = t13806 * t732;
    let t13808 = 0.18311447306006545054e-3 * t13807;
    let t13809 = t13623 - t9954 + t9956 + t13624 - t12754 - t12757 + t13631 - t12769 - t9972 - t13637 - t9980 + t13645 + t10019 + t12780 - t10028 - t13808;
    (t13808, t13809)
}
