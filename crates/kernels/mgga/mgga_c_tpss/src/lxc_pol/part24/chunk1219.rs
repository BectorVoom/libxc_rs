//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1219/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1219<F: Float>(t1338: F, t19462: F, t21021: F, t21171: F, t21222: F, t21224: F, t21226: F, t21229: F, t21231: F, t21233: F, t4674: F, t5514: F, t94: F, t1689: F, t6103: F, t6106: F) -> (F, F, F, F) {
    let t21234 = 4.0 * t1338 * t19462 + 2.0 * t4674 * t5514 + 2.0 * t21021 + t21171 + t21222 + t21224 + t21226 + t21229 + t21231 + t21233;
    let t21236 = t94 * t4674;
    let t21238 = 2.0 * t21236 * t1689;
    let t21240 = 4.0 * t6103 * t6106;
    (t21234, t21236, t21238, t21240)
}
