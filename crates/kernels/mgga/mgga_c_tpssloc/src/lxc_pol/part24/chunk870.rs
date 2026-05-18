//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 870/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk870<F: Float>(t761: F, t9905: F, t9820: F, t9824: F, t9881: F, t9884: F, t9887: F, t9890: F, t9894: F, t9896: F, t9900: F, t9903: F) -> (F, F) {
    let t9907 = F::new(0.35089341735807877242e1) * t761 * t9905;
    let t9908 = -t9820 - t9824 + t9881 - t9884 + t9887 + t9890 - t9894 + t9896 + t9900 - t9903 + t9907;
    (t9907, t9908)
}
