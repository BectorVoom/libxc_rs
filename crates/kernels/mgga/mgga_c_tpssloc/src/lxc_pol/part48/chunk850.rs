//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 850/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk850<F: Float>(t31419: F, t6553: F, t1880: F, t225: F, t8544: F, t6547: F, t8548: F, t25168: F, t2597: F, t2713: F, t30673: F, t30748: F, t31407: F, t31409: F, t31416: F, t6663: F, t7087: F, t855: F, t8553: F, t866: F) -> (F, F, F) {
    let t31420 = t6553 * t31419;
    let t31421 = t1880 * t31420;
    let t31423 = t8544 * t225;
    let t31425 = t6547 * t8548;
    let t31426 = F::cast_from(0.19190897446562641759e-1_f64) * t31425;
    let t31427 = -t30673 - t7087 * t6663 + t31407 + F::new(2.0) * t855 * t31409 + F::new(2.0) * t2597 * t8553 + F::new(2.0) * t2713 * t8553 - F::new(6.0) * t25168 * t31416 - F::cast_from(0.82246703342411321825e-2_f64) * t31421 - t31423 * t866 + t30748 + t31426;
    (t31420, t31423, t31427)
}
