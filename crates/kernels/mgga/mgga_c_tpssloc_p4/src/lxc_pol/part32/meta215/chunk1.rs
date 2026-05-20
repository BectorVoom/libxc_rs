//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1023/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1023<F: Float>(t5742: F, t932: F, t2868: F, t2875: F, t4335: F, t4384: F, t5679: F, t5683: F, t5687: F, t5699: F, t5706: F, t5712: F, t5714: F, t5718: F, t5721: F, t5724: F) -> (F, F) {
    let t5743 = t5742 * t932;
    let t5758 = -F::new(0.17648625e1) * t5699 + F::new(0.3529725e1) * t5706 + t2868 + F::cast_from(0.34431666666666666666e0_f64) * t4335 - F::cast_from(0.34431666666666666667e0_f64) * t5679 + F::new(0.103295e1) * t5683 - F::new(0.516475e0) * t5687 + F::new(0.31558125e0) * t5712 + F::new(0.6311625e0) * t5714 + t2875 + F::cast_from(0.13892666666666666667e0_f64) * t4384 - F::cast_from(0.34731666666666666667e-1_f64) * t5718 + F::new(0.20839e0) * t5721 - F::new(0.104195e0) * t5724;
    (t5743, t5758)
}
