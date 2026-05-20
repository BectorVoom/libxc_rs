//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 802/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk802<F: Float>(t24702: F, t24756: F, t466: F, t24574: F, t7368: F, t2148: F, t3477: F, t1186: F, t7381: F, t3427: F, t2121: F, t225: F, t24594: F) -> (F, F, F, F, F, F, F) {
    let t24757 = t24702 + t24756;
    let t24758 = t466 * t24757;
    let t24760 = t24574 * t7368;
    let t24762 = t3477 * t2148;
    let t24765 = t1186 * t7381;
    let t24771 = t3427 * t2148;
    let t24773 = F::cast_from(0.18277045187202515961e-2_f64) * t2121 * t24771;
    let t24776 = t24594 * t225;
    (t24757, t24758, t24760, t24762, t24765, t24773, t24776)
}
