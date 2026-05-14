//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1291/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1291<F: Float>(t21110: F, t5706: F, t1760: F, t1778: F, t44034: F, t2056: F, t21211: F, t3499: F, t4341: F, t6112: F, t626: F, t13546: F, t94: F, t1689: F, t21236: F, t5522: F) -> (F, F, F, F, F, F, F) {
    let t69373 = t5706 * t21110;
    let t69375 = t1760 * t1778 * t44034;
    let t69377 = 4.0 * t2056 * t21211;
    let t69379 = 4.0 * t3499 * t21211;
    let t69382 = 4.0 * t626 * t4341 * t6112;
    let t69383 = t94 * t13546;
    let t69385 = 2.0 * t69383 * t1689;
    let t69388 = 2.0 * t21236 * t5522;
    (t69373, t69375, t69377, t69379, t69382, t69385, t69388)
}
