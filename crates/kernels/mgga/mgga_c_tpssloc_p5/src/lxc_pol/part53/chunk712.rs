//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 712/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk712<F: Float>(t2039: F, t7042: F, t8446: F, t8711: F, t8717: F, t88: F, t8463: F, t8468: F) -> (F, F) {
    let t8780 = F::new(4.0) * t2039 * t7042 + F::new(2.0) * t8717 * t88 + t8446 + t8711;
    let t8788 = F::cast_from(0.32298204875312312682e-2_f64) * t8463 + t8468 / F::new(384.0);
    (t8780, t8788)
}
