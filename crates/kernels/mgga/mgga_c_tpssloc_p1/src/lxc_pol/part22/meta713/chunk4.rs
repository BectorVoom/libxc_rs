//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2316/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2316<F: Float>(t21038: F, t225: F, t10110: F, t1527: F, t1528: F, t17049: F, t17057: F, t17064: F, t17092: F, t21013: F, t21049: F, t21054: F, t259: F, t2597: F, t2713: F, t2718: F, t40890: F, t4147: F, t4273: F, t4300: F, t5636: F, t5657: F, t59466: F, t59537: F, t798: F, t855: F, t865: F, t866: F) -> F {
    let t67305 = t21038 * t225;
    let t67322 = -F::new(18.0) * t10110 * t4300 * t5636 * t855 + F::new(6.0) * t1527 * t17049 * t2718 * t855 + F::new(24.0) * t21049 * t40890 * t855 * t865 + F::new(6.0) * t2718 * t4300 * t5657 * t855 + t21013 * t259 * t798 - F::new(3.0) * t1528 * t59466 - F::new(3.0) * t1528 * t59537 + F::new(6.0) * t17057 * t4147 - F::new(18.0) * t17064 * t4147 + F::new(12.0) * t17092 * t4273 + F::new(6.0) * t21054 * t2597 + F::new(6.0) * t21054 * t2713 - F::new(3.0) * t67305 * t866;
    t67322
}
