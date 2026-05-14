//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 679/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk679<F: Float>(t26200: F, t225: F, t7910: F, t26231: F, t26251: F, t26255: F, t26266: F, t26361: F, t7919: F, t1824: F, t2085: F, t26393: F, t26406: F, t26429: F, t1338: F, t7918: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t26993 = 0.38381794893125283518e-1 * t26200;
    let t27009 = t7910 * t225;
    let t27012 = 7.0 / 1152.0 * t26231;
    let t27019 = 7.0 / 1152.0 * t26251;
    let t27022 = 7.0 / 288.0 * t26255;
    let t27027 = 7.0 / 72.0 * t26266;
    let t27067 = 0.38381794893125283518e-1 * t26361;
    let t27068 = t7919 * t225;
    let t27074 = t2085 * t1824;
    let t27082 = 0.16449340668482264365e-1 * t26393;
    let t27088 = 0.38381794893125283518e-1 * t26406;
    let t27096 = 0.38381794893125283518e-1 * t26429;
    let t27097 = t1338 * t7918;
    (t26993, t27009, t27012, t27019, t27022, t27027, t27067, t27068, t27074, t27082, t27088, t27096, t27097)
}
