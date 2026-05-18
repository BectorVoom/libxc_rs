//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1027/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1027<F: Float>(t4724: F, t8313: F, t14169: F, t3628: F, t3630: F, t10590: F, t2175: F, t4722: F, t226: F, t3610: F, t3629: F, t2169: F, t4761: F) -> (F, F, F, F, F) {
    let t14220 = t8313 * t4724;
    let t14223 = t3628 * t14169 * t3630;
    let t14229 = t2175 * t10590 * t4722;
    let t14232 = t226 * t3610;
    let t14234 = t2175 * t3629 * t14232;
    let t14238 = t2169 * t4761;
    (t14220, t14223, t14229, t14234, t14238)
}
