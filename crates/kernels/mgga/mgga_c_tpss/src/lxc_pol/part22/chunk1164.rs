//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1164/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1164<F: Float>(t3342: F, t4484: F, t1248: F, t12810: F, t774: F, t1646: F, t9994: F, t10137: F, t4405: F, t1206: F, t4408: F, t762: F) -> (F, F, F, F, F) {
    let t13013 = F::new(7.0) / F::new(576.0) * t3342 * t4484;
    let t13015 = t1248 * t774 * t12810;
    let t13018 = t9994 * t1646;
    let t13021 = F::new(7.0) / F::new(24.0) * t10137 * t4405;
    let t13023 = t762 * t4408 * t1206;
    (t13013, t13015, t13018, t13021, t13023)
}
