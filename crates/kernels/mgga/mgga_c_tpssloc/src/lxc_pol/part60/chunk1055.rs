//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1055/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1055<F: Float>(t113875: F, t116106: F, t116115: F, t117447: F, t117451: F, t117527: F, t122945: F, t122976: F, t124755: F, t124805: F, t124807: F, t124834: F, t124838: F, t126065: F, t126073: F, t129093: F, t129096: F, t1433: F, t31864: F, t32331: F, t34126: F, t5398: F, t8308: F, t8825: F) -> F {
    let t130439 = -F::new(5.0) / F::new(72.0) * t129093 * t8825 - F::new(5.0) / F::new(36.0) * t129096 * t8825 + F::new(10.0) / F::new(27.0) * t124805 + F::new(10.0) / F::new(27.0) * t124807 - F::new(10.0) / F::new(3.0) * t116106 * t117447 * t126065 + F::new(10.0) / F::new(9.0) * t31864 * t117451 * t126073 + F::new(5.0) / F::new(9.0) * t122976 * t34126 + F::new(5.0) / F::new(3.0) * t116115 * t113875 * t124755 * t1433 + F::new(5.0) / F::new(9.0) * t122945 * t34126 + F::new(5.0) / F::new(18.0) * t31864 * t8308 * t32331 * t5398 - F::new(20.0) / F::new(9.0) * t124834 + F::new(20.0) / F::new(27.0) * t124838 - t117527;
    t130439
}
