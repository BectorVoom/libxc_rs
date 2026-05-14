//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1113/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1113<F: Float>(t25: F, t265: F, t394: F, t122917: F, t1874: F, t2113: F, t4072: F, t33690: F, t6525: F, t31832: F, t7756: F, t25992: F, t8690: F, t24991: F, t119677: F, t118965: F, t1409: F, t31823: F, t33750: F, t3966: F, t40: F, t607: F, t8678: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t122918 = t122917 * t1874;
    let t122920 = t2113 * t4072;
    let t122921 = t122920 * t1874;
    let t122923 = t33690 * t6525;
    let t122925 = t31832 * t7756;
    let t123027 = t8690 * t25992;
    let t123028 = t8690 * t24991;
    let t123037 = piecewise3(t395, 0.0, t119677);
    let t123044 = piecewise3(t115, t118965, t123037 * t40 / 2.0 + t31823 * t1409 / 2.0 + t33750 * t607 / 2.0 + t8678 * t3966 / 2.0);
    (t122918, t122920, t122921, t122923, t122925, t123027, t123028, t123044)
}
