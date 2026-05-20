//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1456/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1456<F: Float>(t8689: F, t8944: F, t24994: F, t2105: F, t8110: F, t112: F, t34175: F, t105108: F, t117672: F, t120865: F, t120867: F, t120869: F, t120871: F, t1458: F, t24972: F, t27273: F, t27276: F, t27921: F, t31284: F, t671: F, t7056: F, t7956: F, t8508: F) -> (F, F, F, F) {
    let t123194 = t8689 * t8944;
    let t123198 = t8689 * t24994;
    let t124673 = t8110 * t2105;
    let t124676 = t34175 * t112;
    let t124687 = t120865 + t120867 + F::new(0.135e2) * t27921 * t7056 + t31284 + t8508 + F::new(0.135e2) * t124676 * t671 + t120869 + t120871 + F::new(27.0) * t105108 * t7956 + F::new(27.0) * t24972 * t27273 + F::new(27.0) * t24972 * t27276 + F::new(0.135e2) * t117672 * t1458;
    (t123194, t123198, t124673, t124687)
}
