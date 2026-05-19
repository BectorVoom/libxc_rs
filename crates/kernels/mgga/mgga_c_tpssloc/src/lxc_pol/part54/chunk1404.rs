//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1404/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1404<F: Float>(t25: F, t265: F, t394: F, t121907: F, t121949: F, t121283: F, t121798: F, t121833: F, t121865: F, t1409: F, t31478: F, t33513: F, t3966: F, t40: F, t607: F, t8580: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t121950 = t121907 + t121949;
    let t121951 = piecewise3::<F>(t395, F::new(0.0), t121950);
    let t121958 = piecewise3::<F>(t115, t121283 + t121798 + t121833 + t121865, t121951 * t40 / F::new(2.0) + t31478 * t1409 / F::new(2.0) + t33513 * t607 / F::new(2.0) + t8580 * t3966 / F::new(2.0));
    (t121950, t121958)
}
