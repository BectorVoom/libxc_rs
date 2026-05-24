//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1019/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1019<F: Float>(t77723: F, t75662: F, t75664: F, t71544: F, t71545: F, t71546: F, t71551: F, t75648: F, t75652: F, t75654: F, t75658: F, t75666: F, t77713: F, t77715: F, t77717: F, t77719: F, t77720: F, t884: F) -> F {
    let t77724 = F::cast_from(0.99317399751028291929e-5_f64) * t77723;
    let t77725 = F::cast_from(0.3830813990396805546e-4_f64) * t75662;
    let t77726 = F::cast_from(0.1276937996798935182e-4_f64) * t75664;
    let t77727 = F::cast_from(0.93188427318671584245e-2_f64) * t75648 + F::cast_from(0.93188427318671584245e-2_f64) * t75652 - F::cast_from(0.15531404553111930708e-1_f64) * t75654 - F::cast_from(0.15531404553111930708e-1_f64) * t75658 + t77713 + t77715 + t77717 - t77719 + F::cast_from(0.59871208509319042821e-1_f64) * t884 * t77720 + t77724 + t77725 + t77726 + t75666 - t71544 - t71545 + t71546 + t71551;
    t77727
}
