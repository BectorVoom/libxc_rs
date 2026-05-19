//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1221/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1221<F: Float>(t28: F, t265: F, t504: F, t119733: F, t119783: F, t119677: F, t1409: F, t30983: F, t33074: F, t3966: F, t52: F, t607: F, t8435: F, t31222: F, t7685: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F, F) {
    let t29 = t28 <= zeta_threshold;
    let t401 = rho1 <= dens_threshold || t29;
    let t505 = t265 < t504;
    let t119784 = t119733 + t119783;
    let t119785 = piecewise3::<F>(t505, F::new(0.0), t119677);
    let t119792 = piecewise3::<F>(t401, t119784, t119785 * t52 / F::new(2.0) - t30983 * t1409 / F::new(2.0) - t33074 * t607 / F::new(2.0) - t8435 * t3966 / F::new(2.0));
    let t119795 = t7685 * t31222;
    (t119792, t119795)
}
