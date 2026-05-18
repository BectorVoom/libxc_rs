//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1018/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1018<F: Float>(t116065: F, t117447: F, t625: F, t79: F, t641: F, t8513: F, t8663: F, t113824: F, t113864: F, t113876: F, t116075: F, t116082: F, t116106: F, t116124: F, t117451: F, t117461: F, t2241: F, t2244: F, t2307: F, t31857: F, t31860: F, t31864: F, t31868: F, t32328: F, t32340: F, t7246: F, t8824: F) -> F {
    let t117477 = t116065 * t117447;
    let t117480 = t79 * t625;
    let t117483 = t8663 * t8513 * t117480 * t641;
    let t117487 = -F::new(10.0) / F::new(3.0) * t116106 * t117447 * t113864 + F::new(10.0) / F::new(9.0) * t31864 * t117451 * t113876 - F::new(35.0) / F::new(12.0) * t116075 * t8513 * t8824 * t2241 - F::new(20.0) / F::new(9.0) * t117461 + F::new(5.0) / F::new(18.0) * t7246 * t8513 * t8824 * t2244 + F::new(5.0) / F::new(6.0) * t116124 * t32328 - F::new(5.0) / F::new(18.0) * t31857 * t32340 + F::new(5.0) / F::new(6.0) * t116082 * t32328 + F::new(5.0) / F::new(12.0) * t31860 * t8513 * t8824 * t2307 - F::new(5.0) / F::new(9.0) * t113824 * t117477 + F::new(20.0) / F::new(27.0) * t117483 - F::new(5.0) / F::new(18.0) * t31868 * t32340;
    t117487
}
