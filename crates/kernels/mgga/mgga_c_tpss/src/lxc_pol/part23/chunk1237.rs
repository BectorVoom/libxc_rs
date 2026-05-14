//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1237/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1237<F: Float>(t1273: F, t19623: F, t19624: F, t19628: F, t19630: F, t19634: F, t19635: F, t2056: F, t20957: F, t20969: F, t20981: F, t3499: F, t3538: F, t3542: F, t544: F, t5986: F, t624: F, t626: F, t646: F, t6486: F, t6540: F, t6544: F) -> (F,) {
    let t20983 = t1273 * t6544 - 2.0 * t2056 * t6486 - 2.0 * t20957 * t646 - 2.0 * t20969 * t626 + t20981 * t544 - 2.0 * t3499 * t6486 - 2.0 * t3538 * t5986 - 2.0 * t3542 * t5986 - t624 * t6540 + t19623 - t19624 - t19628 + t19630 + t19634 - t19635;
    (t20983,)
}
