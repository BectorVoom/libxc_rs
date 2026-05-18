//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 231/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk231<F: Float>(t40: F, t52: F, t761: F, t763: F, t201: F, t262: F, t73: F, t607: F, t76: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t765 = F::new(0.5848223622634646207e0) * t761 * t763;
    let t766 = t201 * t262;
    let t767 = F::new(1.0) / t73;
    let t770 = piecewise3::<f64>(t146, F::new(0.0), F::new(2.0) / F::new(3.0) * t767 * t607);
    let t771 = F::new(1.0) / t76;
    let t774 = piecewise3::<f64>(t150, F::new(0.0), -F::new(2.0) / F::new(3.0) * t771 * t607);
    let t776 = t770 / F::new(2.0) + t774 / F::new(2.0);
    (t765, t766, t767, t771, t776)
}
