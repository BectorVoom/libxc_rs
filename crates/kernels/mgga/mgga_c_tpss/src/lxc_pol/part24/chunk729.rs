//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 729/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk729<F: Float>(t3647: F, t162: F, t4740: F, t187: F, t2208: F, t2217: F, t2224: F, t2281: F, t2285: F, t2292: F, t2302: F, t2310: F, t2333: F, t2347: F, t2351: F, t4680: F, t4682: F, t4685: F, t4686: F, t4687: F, t4727: F, t4742: F) -> (F, F, F, F) {
    let t4743 = 2.0 * t3647;
    let t4744 = t4740 * t162;
    let t4746 = 0.19751673498613801407e-1 * t4744 * t187;
    let t4747 = -t2208 - t2217 + t2224 + t2333 + t2302 + t2310 - t2292 + t4727 - t2281 + t2347 - t2285 - t4687 + t4742 + t4680 + t4682 + t2351 + t4743 + t4746 + t4685 - t4686;
    (t4743, t4744, t4746, t4747)
}
