//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1355/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1355<F: Float>(t15499: F, t6013: F, t15522: F, t19090: F, t15515: F, t20831: F, t4238: F, t15533: F, t15848: F, t15891: F, t20837: F, t4248: F, t4285: F, t63254: F, t63258: F, t68405: F, t68407: F) -> (F,) {
    let t73335 = t6013 * t15499;
    let t73347 = t19090 * t15522;
    let t73349 = t6013 * t15515;
    let t73351 = t20831 * t4238;
    let t73353 = -t73335 / 1728.0 - t6013 * t15533 / 2304.0 + t63254 * t15891 / 256.0 - t63258 * t15848 / 256.0 + t20837 * t4285 / 108.0 - t20831 * t4248 / 144.0 - t73347 / 2304.0 + 5.0 / 10368.0 * t73349 - t73351 / 216.0 + t68405 + t68407;
    (t73353,)
}
