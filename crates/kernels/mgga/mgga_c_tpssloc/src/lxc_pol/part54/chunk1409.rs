//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1409/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1409<F: Float>(t28: F, t265: F, t504: F, t121950: F, t121982: F, t122012: F, t122042: F, t122072: F, t1409: F, t31512: F, t33547: F, t3966: F, t52: F, t607: F, t8591: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t122075 = piecewise3::<f64>(t505, F::new(0.0), t121950);
    let t122082 = piecewise3::<f64>(t401, t121982 + t122012 + t122042 + t122072, t122075 * t52 / F::new(2.0) - t31512 * t1409 / F::new(2.0) - t33547 * t607 / F::new(2.0) - t8591 * t3966 / F::new(2.0));
    t122082
}
