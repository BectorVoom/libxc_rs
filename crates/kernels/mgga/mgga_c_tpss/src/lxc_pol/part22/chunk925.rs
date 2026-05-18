//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 925/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk925<F: Float>(t2202: F, t862: F, t2522: F, t673: F, t2516: F, t235: F, t2697: F, t2519: F, t262: F, t265: F, t5543: F, t599: F) -> (F, F, F, F, F, F, F, F) {
    let t8627 = t2202 * t862;
    let t8629 = t673 * t2522;
    let t8631 = t673 * t2516;
    let t8633 = t235 * t2697;
    let t8647 = t673 * t2519;
    let t8660 = t262 * t5543 * t265;
    let t8661 = F::new(0.93011851851851851854e0) * t8660;
    let t8662 = t599 * t235;
    (t8627, t8629, t8631, t8633, t8647, t8660, t8661, t8662)
}
