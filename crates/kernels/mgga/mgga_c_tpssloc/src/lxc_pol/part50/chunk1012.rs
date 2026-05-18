//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1012/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1012<F: Float>(t1332: F, t2013: F, t22693: F, t22707: F, t26379: F, t26381: F, t26386: F, t26390: F, t26393: F, t26398: F, t26401: F, t26404: F, t26406: F, t26412: F, t26416: F, t26419: F, t26424: F, t26427: F, t26429: F, t5230: F, t5344: F, t544: F, t7747: F) -> F {
    let t26431 = F::new(0.16449340668482264365e-1) * t26379 + F::new(0.38381794893125283518e-1) * t26381 - t22693 + t5230 * t2013 - F::new(0.16449340668482264365e-1) * t26386 - F::new(0.16449340668482264365e-1) * t26390 + F::new(0.82246703342411321825e-2) * t26393 - F::new(0.16449340668482264365e-1) * t26398 + t1332 * t7747 + t544 * t26401 - t5344 * t26404 + F::new(0.19190897446562641759e-1) * t26406 + F::new(0.41123351671205660912e-2) * t22707 - F::new(0.82246703342411321825e-2) * t26412 + F::new(0.16449340668482264365e-1) * t26416 - F::new(0.82246703342411321825e-2) * t26419 + F::new(0.16449340668482264365e-1) * t26424 + F::new(0.41123351671205660912e-2) * t26427 - F::new(0.19190897446562641759e-1) * t26429;
    t26431
}
