//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 710/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk710<F: Float>(t22690: F, t6968: F, t22642: F, t1351: F, t1372: F, t550: F, t6976: F, t1992: F, t12272: F, t268: F, t534: F, t6559: F) -> (F, F, F, F, F) {
    let t22691 = t22690 * t6968;
    let t22692 = t22642 * t22691;
    let t22693 = F::new(0.82246703342411321824e-2) * t22692;
    let t22694 = t1372 * t1351;
    let t22695 = t22694 * t550;
    let t22696 = t6976 * t22695;
    let t22697 = t1992 * t22696;
    let t22699 = t12272 * t550;
    let t22700 = t6976 * t22699;
    let t22701 = t1992 * t22700;
    let t22704 = t6559 * t534 * t268;
    (t22692, t22693, t22697, t22701, t22704)
}
