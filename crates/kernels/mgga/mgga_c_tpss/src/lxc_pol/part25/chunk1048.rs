//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1048/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1048<F: Float>(t15595: F, t926: F, t15232: F, t15355: F, t15361: F, t15363: F, t15365: F, t15411: F, t15413: F, t15417: F, t15421: F, t15426: F, t15443: F, t15446: F, t15448: F, t15465: F, t15467: F, t15473: F, t15475: F) -> (F, F) {
    let t15596 = t926 * t15595;
    let t15599 = -t15232 - t15355 + t15361 - t15363 + t15365 + t15411 + t15413 - t15417 + t15421 - t15426 + t15443 + t15446 + t15448 - t15465 - t15467 - t15473 - t15475;
    (t15596, t15599)
}
