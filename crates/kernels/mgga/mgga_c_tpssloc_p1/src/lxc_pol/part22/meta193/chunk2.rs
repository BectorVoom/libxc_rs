//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1141/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1141<F: Float>(t2798: F, t5698: F, t2802: F, t4335: F, t5679: F, t5683: F, t5687: F) -> (F, F) {
    let t5699 = t2798 * t5698;
    let t5705 = t2802 + F::new(2.0) / F::new(9.0) * t4335 - F::new(2.0) / F::new(9.0) * t5679 + F::new(2.0) / F::new(3.0) * t5683 - t5687 / F::new(3.0);
    (t5699, t5705)
}
