//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 262/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk262<F: Float>(t40: F, t52: F, t185: F, t607: F, t707: F, t73: F, t76: F, zeta_threshold: F) -> (F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t708 = t185 * t607;
    let t710 = F::new(4.0) * t707 * t708;
    let t713 = piecewise3::<F>(t146, F::new(0.0), F::new(4.0) / F::new(3.0) * t73 * t607);
    let t716 = piecewise3::<F>(t150, F::new(0.0), -F::new(4.0) / F::new(3.0) * t76 * t607);
    let t717 = t713 + t716;
    (t708, t710, t717)
}
