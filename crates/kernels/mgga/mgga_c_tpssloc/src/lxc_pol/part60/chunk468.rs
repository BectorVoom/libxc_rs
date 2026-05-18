//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 468/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk468<F: Float>(t291: F, t5689: F, t1557: F, t4354: F, t1556: F, t913: F, t2792: F, t1547: F, t2798: F, t2802: F, t4335: F, t5679: F, t5683: F, t5687: F) -> (F, F, F, F, F, F, F) {
    let t5691 = F::new(0.621814e-1) * t5689 * t291;
    let t5693 = F::new(2.0) * t4354 * t1557;
    let t5694 = t1556 * t1556;
    let t5695 = t5694 * t913;
    let t5697 = F::new(2.0) * t2792 * t5695;
    let t5698 = t1547 * t1547;
    let t5699 = t2798 * t5698;
    let t5705 = t2802 + F::new(2.0) / F::new(9.0) * t4335 - F::new(2.0) / F::new(9.0) * t5679 + F::new(2.0) / F::new(3.0) * t5683 - t5687 / F::new(3.0);
    (t5691, t5693, t5694, t5697, t5698, t5699, t5705)
}
