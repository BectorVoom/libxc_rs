//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1079/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1079<F: Float>(t75519: F, t69828: F, t73375: F, t75458: F, t75461: F, t75490: F, t75495: F, t75500: F, t77630: F, t77631: F, t77633: F, t77634: F, t77635: F, t77636: F, t77641: F, t77642: F, t77643: F) -> F {
    let t80256 = F::cast_from(0.24527028530061914062e-5_f64) * t75519;
    let t80257 = F::cast_from(0.10511583655740820312e-5_f64) * t75458 - F::cast_from(0.10511583655740820312e-5_f64) * t75461 - t77630 - t77631 + t77633 + t77634 - t77635 + t77636 - t73375 - t75490 - t75495 + t75500 + t77641 - t77642 + t77643 - t80256 - t69828;
    t80257
}
