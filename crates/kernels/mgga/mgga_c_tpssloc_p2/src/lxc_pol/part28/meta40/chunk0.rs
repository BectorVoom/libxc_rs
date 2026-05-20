//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 270/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk270<F: Float>(t40: F, t607: F, t767: F, t76: F, zeta_threshold: F) -> (F, F) {
    let t146 = t40 <= zeta_threshold;
    let t770 = piecewise3::<F>(t146, F::new(0.0), F::new(2.0) / F::new(3.0) * t767 * t607);
    let t771 = F::new(1.0) / t76;
    (t770, t771)
}
