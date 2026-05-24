//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 585/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk585<F: Float>(t344: F, t830: F, t3839: F, t7634: F, t7625: F, t7628: F, t7629: F, t7631: F, t7636: F, t7640: F, t7643: F, t7647: F, t7649: F, t7652: F, t7654: F, t7656: F, t7658: F, t7660: F) -> (F, F) {
    let t7662 = t344 * t830;
    let t7663 = F::cast_from(0.64905642291407286545e-3_f64) * t7662;
    let t7664 = t3839 * t7634;
    let t7666 = -F::cast_from(0.21241846568096930142e-2_f64) * t7625 - t7628 + F::cast_from(0.34093327067806677162e-2_f64) * t7629 - F::cast_from(0.45457769423742236216e-2_f64) * t7631 + F::cast_from(0.68186654135613354324e-2_f64) * t7636 - t7640 + F::cast_from(0.22728884711871118108e-1_f64) * t7643 + t7647 + F::cast_from(0.45360193192290319575e-3_f64) * t7649 - t7652 + F::cast_from(0.84672360625608596544e-3_f64) * t7654 + F::cast_from(0.56448240417072397695e-3_f64) * t7656 + F::cast_from(0.5987120850931904282e-1_f64) * t7658 - F::cast_from(0.99785347515531738034e-2_f64) * t7660 - t7663 - F::cast_from(0.13276154105060581339e-2_f64) * t7664;
    (t7663, t7666)
}
