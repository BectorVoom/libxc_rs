//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1061/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1061<F: Float>(t11584: F, t2740: F, t3944: F, t8983: F, t11562: F, t11565: F, t11572: F, t11579: F, t1461: F, t3963: F, t8450: F, t8509: F, t8514: F, t8985: F, t8989: F, t8998: F, t9004: F) -> F {
    let t11586 = t2740 * t11584 / F::new(3456.0);
    let t11588 = t8983 * t3944;
    let t11590 = t2740 * t11588 / F::new(3456.0);
    let t11591 = t8985 / F::new(3456.0) + F::new(11.0) / F::new(324.0) * t8450 * t1461 - t11562 + t8998 / F::new(864.0) + t2740 * t11565 / F::new(4608.0) - t8509 * t11572 / F::new(2304.0) + t8514 * t11579 / F::new(1152.0) - t8989 * t3963 / F::new(432.0) + t11586 - t9004 / F::new(3456.0) + t11590;
    t11591
}
