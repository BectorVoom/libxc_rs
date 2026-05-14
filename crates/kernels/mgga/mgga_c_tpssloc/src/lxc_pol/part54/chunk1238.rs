//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1238/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1238<F: Float>(t121320: F, t121343: F, t121364: F, t121393: F, t121411: F, t121440: F, t121462: F, t121479: F, t121623: F, t121643: F, t121668: F, t121691: F, t121711: F, t121725: F, t121747: F, t121770: F) -> (F,) {
    let t121774 = t121320 + t121343 + t121364 + t121393 + t121411 + t121440 + t121462 + t121479 + t121623 + t121643 + t121668 + t121691 + t121711 + t121725 + t121747 + t121770;
    (t121774,)
}
