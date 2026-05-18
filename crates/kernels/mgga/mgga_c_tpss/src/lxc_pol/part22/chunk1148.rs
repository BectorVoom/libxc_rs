//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1148/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1148<F: Float>(t10019: F, t10025: F, t10028: F, t10236: F, t1196: F, t1206: F, t12754: F, t12755: F, t12756: F, t12757: F, t12759: F, t12760: F, t12764: F, t12769: F, t12770: F, t12775: F, t12779: F, t12780: F, t12810: F, t1625: F, t198: F, t3183: F, t3234: F, t4528: F, t4532: F, t9972: F, t9980: F) -> F {
    let t12814 = -F::new(3.0) * t10236 * t1625 * t3183 + F::new(3.0) * t1196 * t12810 * t198 + F::new(12.0) * t1206 * t12760 * t4532 + F::new(3.0) * t3183 * t3234 * t4528 + F::new(6.0) * t12764 * t4532 - t10019 + t10025 - t10028 - t12754 - t12755 - t12756 + t12757 + t12759 - t12769 - t12770 + t12775 - t12779 + t12780 - t9972 - t9980;
    t12814
}
