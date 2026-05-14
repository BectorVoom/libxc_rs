//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 979/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk979<F: Float>(t5: F, t32578: F, t9239: F, t33: F, t8854: F, t2240: F, t7254: F, t8307: F, t8513: F, t31000: F, t31006: F, t31013: F, t31024: F, t8663: F, t8856: F, t112: F, t671: F, t8913: F) -> (F, F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t32579 = t9239 * t32578;
    let t32582 = t33 * t8854;
    let t32583 = t2240 * t32582;
    let t32587 = t8513 * t8307 * t7254;
    let t32590 = t2240 * t32578;
    let t32594 = piecewise3(t8, 0.0, 5.0 / 144.0 * t31000 * t8856 - 5.0 / 24.0 * t32579 * t31006 - 5.0 / 36.0 * t32583 * t31013 + 5.0 / 72.0 * t8663 * t32587 + 5.0 / 72.0 * t32590 * t31024);
    let t32595 = t32594 * t112;
    let t32605 = t8913 * t671;
    (t32579, t32582, t32583, t32587, t32590, t32594, t32595, t32605)
}
