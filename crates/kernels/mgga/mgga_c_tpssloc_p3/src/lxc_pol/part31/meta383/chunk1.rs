//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1353/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1353<F: Float>(t10817: F, t5695: F, t2787: F, t5727: F, t10296: F, t10556: F, t10675: F, t10676: F, t13551: F, t13552: F, t13563: F, t13567: F, t17173: F, t17180: F, t17185: F) -> (F, F, F) {
    let t17377 = F::cast_from(2.0_f64) * t10817 * t5695;
    let t17379 = F::cast_from(1.0_f64) * t2787 * t5727;
    let t17398 = F::cast_from(0.11958666666666666667e1_f64) * t17173 - t13551 + F::cast_from(0.36514074074074074073e-1_f64) * t13552 + F::cast_from(0.13287407407407407407e0_f64) * t13563 - t13567 - F::cast_from(0.91285185185185185187e-1_f64) * t10296 - t10675 - t10676 - F::cast_from(0.19931111111111111111e0_f64) * t17180 + F::cast_from(0.59793333333333333334e0_f64) * t17185 - F::cast_from(0.13287407407407407408e0_f64) * t10556;
    (t17377, t17379, t17398)
}
