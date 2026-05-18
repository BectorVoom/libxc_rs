//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1217/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1217<F: Float>(t25: F, t265: F, t394: F, t119639: F, t119676: F, t119608: F, t118965: F, t1409: F, t30953: F, t33044: F, t3966: F, t40: F, t607: F, t8425: F, t23788: F, t4255: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t119677 = t119639 + t119676;
    let t119678 = piecewise3::<f64>(t395, t119608, t119677);
    let t119685 = piecewise3::<f64>(t115, t118965, t119678 * t40 / F::new(2.0) + t30953 * t1409 / F::new(2.0) + t33044 * t607 / F::new(2.0) + t8425 * t3966 / F::new(2.0));
    let t119691 = t23788 * t4255;
    (t119677, t119685, t119691)
}
