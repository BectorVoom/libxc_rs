//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 746/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk746<F: Float>(t1860: F, t26959: F, t26198: F, t12020: F, t2091: F, t26200: F, t225: F, t7910: F, t26231: F, t26251: F, t26255: F, t26266: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26960 = t1860 * t26959;
    let t26988 = F::cast_from(0.16449340668482264365e-1_f64) * t26198;
    let t26989 = t12020 * t2091;
    let t26993 = F::cast_from(0.38381794893125283518e-1_f64) * t26200;
    let t27009 = t7910 * t225;
    let t27012 = F::new(7.0) / F::new(1152.0) * t26231;
    let t27019 = F::new(7.0) / F::new(1152.0) * t26251;
    let t27022 = F::new(7.0) / F::new(288.0) * t26255;
    let t27027 = F::new(7.0) / F::new(72.0) * t26266;
    (t26960, t26988, t26989, t26993, t27009, t27012, t27019, t27022, t27027)
}
