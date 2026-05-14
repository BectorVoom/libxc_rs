//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 853/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk853<F: Float>(t12045: F, t3824: F, t592: F, t11976: F, t11978: F, t11980: F, t11982: F, t11984: F, t12044: F, t9457: F, t9476: F, t9484: F, t9780: F, t1285: F, t2221: F, t1287: F) -> (F, F, F, F, F) {
    let t12046 = 144.0 * t12045;
    let t12048 = 12.0 * t592 * t3824;
    let t12049 = -t9457 + t9476 + t9484 + t11976 - t11978 - t11980 - t11982 - t11984 + t9780 + t12044 - t12046 - t12048;
    let t12050 = t2221 * t1285;
    let t12051 = 36.0 * t12050;
    let t12052 = t2221 * t1287;
    (t12046, t12048, t12049, t12051, t12052)
}
