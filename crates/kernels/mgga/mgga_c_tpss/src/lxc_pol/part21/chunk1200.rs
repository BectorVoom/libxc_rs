//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1200/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1200<F: Float>(t1689: F, t19305: F, t3537: F, t94: F, t5522: F, t6103: F, t1753: F, t6228: F, t645: F, t5532: F, t2056: F, t6106: F, t3499: F, t1688: F, t4341: F, t626: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19307 = 2.0 * t19305 * t1689;
    let t19308 = t94 * t3537;
    let t19310 = 2.0 * t19308 * t1689;
    let t19312 = 2.0 * t6103 * t5522;
    let t19315 = t1753 * t3537;
    let t19318 = t6228 * t645;
    let t19322 = 2.0 * t6103 * t5532;
    let t19324 = 2.0 * t2056 * t6106;
    let t19326 = 2.0 * t3499 * t6106;
    let t19327 = t4341 * t1688;
    let t19329 = 2.0 * t626 * t19327;
    (t19307, t19308, t19310, t19312, t19315, t19318, t19322, t19324, t19326, t19327, t19329)
}
