//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2004/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2004<F: Float>(t28: F, t265: F, t504: F, t93100: F, t12606: F, t1409: F, t2071: F, t2250: F, t24420: F, t26862: F, t3966: F, t52: F, t607: F, t7150: F, t7884: F, t93144: F, t93181: F, t93211: F, t93246: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t93249 = piecewise3::<F>(t505, F::new(0.0), t93100);
    let t93261 = piecewise3::<F>(t401, t93144 + t93181 + t93211 + t93246, t93249 * t52 / F::new(2.0) - t26862 * t607 - t7884 * t2250 / F::new(2.0) - t24420 * t1409 / F::new(2.0) - t7150 * t3966 - t2071 * t12606 / F::new(2.0));
    t93261
}
