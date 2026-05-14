//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1118/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1118<F: Float>(t10019: F, t10025: F, t12751: F, t12754: F, t12755: F, t12756: F, t12757: F, t12759: F, t12769: F, t12770: F, t12775: F, t12779: F, t12780: F, t9956: F, t9972: F, t9980: F) -> (F,) {
    let t12906 = t9956 + t12751 - t12754 - t12755 - t12756 + t12757 + t12759 - t12769 - t12770 - t9972 + t12775 - t9980 - t12779 - t10019 + t12780 + t10025;
    (t12906,)
}
