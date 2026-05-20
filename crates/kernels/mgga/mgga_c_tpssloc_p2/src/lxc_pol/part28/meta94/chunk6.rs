//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 588/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk588<F: Float>(t25: F, t265: F, t504: F, t1877: F, t2058: F, t2064: F, t40: F, t2057: F, t28: F, t2063: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t505 = t265 < t504;
    let t2067 = piecewise3::<F>(t115, t1877 * t2058 / F::new(2.0), t2064 * t40 / F::new(2.0));
    let t2068 = t2057 * t28;
    let t2071 = piecewise3::<F>(t505, F::new(0.0), t2063);
    (t2067, t2068, t2071)
}
