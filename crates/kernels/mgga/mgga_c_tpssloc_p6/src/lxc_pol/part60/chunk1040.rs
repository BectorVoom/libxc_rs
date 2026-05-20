//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1040/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1040<F: Float>(t102801: F, t1992: F, t550: F, t6976: F, t102587: F, t102562: F, t1985: F, t1998: F, t214: F, t29286: F, t115391: F, t122460: F, t122462: F, t127356: F, t127357: F, t128626: F, t1814: F, t33291: F, t544: F, t553: F) -> F {
    let t128823 = t1992 * t6976 * t102801 * t550;
    let t128829 = t1992 * t6976 * t102587 * t550;
    let t128833 = t1992 * t6976 * t102562 * t550;
    let t128839 = t1985 * t214 * t1998 * t29286;
    let t128841 = t127356 + F::cast_from(0.82246703342411321824e-2_f64) * t122460 + t127357 + F::cast_from(0.38381794893125283518e-1_f64) * t122462 - F::cast_from(0.82246703342411321825e-2_f64) * t128823 + F::new(2.0) * t1814 * t33291 - t115391 - F::cast_from(0.82246703342411321825e-2_f64) * t128829 - F::cast_from(0.16449340668482264365e-1_f64) * t128833 + t544 * t553 * t128626 + F::cast_from(0.82246703342411321825e-2_f64) * t128839;
    t128841
}
