//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 743/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk743<F: Float>(t198: F, t207: F, t2224: F, t2281: F, t2285: F, t2436: F, t4680: F, t4682: F, t4685: F, t4686: F, t4687: F, t4701: F, t4742: F, t4802: F, t4806: F, t740: F, t823: F) -> (F,) {
    let t4810 = -t198 * t207 * t2436 * t4806 + t198 * t207 * t4802 * t823 + 3.0 * t198 * t4701 * t740 + t2224 - t2281 - t2285 + t4680 + t4682 + t4685 - t4686 - t4687 + t4742;
    (t4810,)
}
