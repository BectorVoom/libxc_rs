//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1361/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1361<F: Float>(t63749: F, t65080: F, t65132: F, t65475: F, t65523: F, t65918: F, t66018: F, t66061: F, t4549: F, t5776: F, t13220: F, t547: F, t5772: F, t1279: F, t20113: F, t5773: F) -> (F, F, F, F, F) {
    let t66064 = t63749 + t65080 + t65132 + t65475 + t65523 + t65918 + t66018 + t66061;
    let t66068 = 6.0 * t4549 * t5776;
    let t66073 = 6.0 * t547 * t5772 * t13220;
    let t66075 = 12.0 * t1279 * t20113;
    let t66077 = 12.0 * t4549 * t5773;
    (t66064, t66068, t66073, t66075, t66077)
}
