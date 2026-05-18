//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1083/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1083<F: Float>(t15001: F, t558: F, t15931: F, t275: F, t71544: F, t71545: F, t71546: F, t75648: F, t75652: F, t75654: F, t75658: F, t75666: F, t77713: F, t77715: F, t77717: F, t77719: F, t77724: F, t77725: F, t77726: F, t884: F) -> (F, F) {
    let t80280 = t15001 * t558;
    let t80283 = t275 * t15931;
    let t80284 = F::new(0.93188427318671584242e-2) * t75648 + F::new(0.93188427318671584242e-2) * t75652 - F::new(0.15531404553111930707e-1) * t75654 - F::new(0.15531404553111930707e-1) * t75658 + t77713 + t77715 + t77717 - t77719 + F::new(0.59871208509319042821e-1) * t884 * t80280 + t77724 + t77725 + t77726 + t75666 + t80283 - t71544 - t71545 + t71546;
    (t80280, t80284)
}
