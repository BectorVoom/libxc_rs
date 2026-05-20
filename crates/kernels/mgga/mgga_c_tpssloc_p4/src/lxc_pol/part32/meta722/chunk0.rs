//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2299/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2299<F: Float>(t1184: F, t6139: F, t1716: F, t1752: F, t17686: F, t2155: F, t24589: F, t24590: F, t24601: F, t24633: F, t24638: F, t254: F, t27406: F, t27412: F, t27549: F, t27747: F, t27774: F, t27775: F, t27786: F, t27799: F, t29816: F, t4945: F, t6140: F, t66860: F, t7283: F, t94349: F, t94458: F, t94503: F, t94584: F, t94676: F) -> (F, F) {
    let t103422 = t6139 * t1184;
    let t103457 = F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t6140 * t24638 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t103422 * t27799 - t94676 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t94503 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t24633 * t29816 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t94584 - F::cast_from(0.87729816898572076612e-1_f64) * t27406 * t27412 - t66860 * t2155 + F::cast_from(0.21932454224643019154e-1_f64) * t27549 * t24601 * t94349 * t17686 - F::cast_from(0.16449340668482264365e-1_f64) * t24589 * t24601 * t27774 * t17686 + F::new(4.0) * t4945 * t27747 - F::new(12.0) * t1752 * t254 * t27786 + F::cast_from(0.73108180748810063845e-2_f64) * t27549 * t94458 * t27775 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24590 * t29816;
    (t103422, t103457)
}
