//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1053/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1053<F: Float>(t25277: F, t25077: F, t25080: F, t23114: F, t23120: F, t24218: F, t24220: F, t24221: F, t25085: F, t25087: F, t25089: F, t25091: F, t25095: F, t25099: F) -> (F, F, F, F) {
    let t26613 = F::new(0.38381794893125283518e-1) * t25277;
    let t26619 = F::new(7.0) / F::new(288.0) * t25077;
    let t26621 = F::new(7.0) / F::new(1152.0) * t25080;
    let t26630 = t24218 - t24220 + t25085 / F::new(384.0) + t25087 / F::new(192.0) - t25089 / F::new(768.0) + t25091 / F::new(192.0) + F::new(0.80745512188280781706e-3) * t25095 + t24221 + F::new(0.24223653656484234512e-2) * t25099 + F::new(0.67287926823567318088e-4) * t23114 - t23120;
    (t26613, t26619, t26621, t26630)
}
