//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 933/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk933<F: Float>(t13042: F, t1912: F, t23249: F, t23252: F, t23254: F, t23262: F, t25230: F, t25233: F, t25330: F, t25339: F, t25343: F, t25346: F, t25348: F, t2597: F, t2713: F, t7517: F, t855: F, t866: F) -> F {
    let t25351 = -F::cast_from(0.16449340668482264365e-1_f64) * t25230 + F::cast_from(2.0_f64) * t855 * t25233 - t855 * t25330 - F::cast_from(0.19190897446562641759e-1_f64) * t23249 + t23252 - F::cast_from(0.41123351671205660912e-2_f64) * t23254 + t23262 + F::cast_from(2.0_f64) * t2597 * t7517 + F::cast_from(2.0_f64) * t2713 * t7517 - F::cast_from(0.16449340668482264365e-1_f64) * t25339 - F::cast_from(0.16449340668482264365e-1_f64) * t25343 + F::cast_from(0.82246703342411321825e-2_f64) * t25346 - t25348 * t866 - t13042 * t1912;
    t25351
}
