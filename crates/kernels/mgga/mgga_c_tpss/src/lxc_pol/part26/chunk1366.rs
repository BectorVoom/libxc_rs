//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1366/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1366<F: Float>(t1665: F, t6556: F, t22198: F, t550: F, t1906: F, t5465: F, t1276: F, t1278: F, t1666: F, t1673: F, t20986: F, t21007: F, t22217: F, t5466: F, t6071: F, t67886: F, t67888: F, t68773: F, t68776: F, t68780: F, t72768: F, t73617: F) -> (F,) {
    let t73620 = t1665 * t6556;
    let t73624 = t22198 * t550;
    let t73626 = t5465 * t1906;
    let t73627 = t5466 * t6071 + t67886 + t67888 + 2.0 * t20986 * t1673 + t1278 * (t72768 + t73617) + 2.0 * t73620 + 2.0 * t1666 * t21007 + t73624 + t68773 + t1276 * t22217 + t68776 + t68780 + t73626;
    (t73627,)
}
