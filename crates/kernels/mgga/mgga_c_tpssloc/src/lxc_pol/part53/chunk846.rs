//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 846/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk846<F: Float>(t26361: F, t225: F, t7919: F, t2085: F, t5210: F, t1824: F, t5250: F, t1352: F, t26393: F, t1825: F, t24116: F, t26406: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27067 = F::new(0.38381794893125283518e-1) * t26361;
    let t27068 = t7919 * t225;
    let t27070 = t5210 * t2085;
    let t27074 = t2085 * t1824;
    let t27075 = t27074 * t5250;
    let t27078 = t27074 * t1352;
    let t27082 = F::new(0.16449340668482264365e-1) * t26393;
    let t27086 = t24116 * t1825;
    let t27088 = F::new(0.38381794893125283518e-1) * t26406;
    (t27067, t27068, t27070, t27074, t27075, t27078, t27082, t27086, t27088)
}
