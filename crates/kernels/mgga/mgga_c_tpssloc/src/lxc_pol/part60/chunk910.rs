//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 910/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk910<F: Float>(t34104: F, t34115: F, t34157: F, t34173: F, t3: F, t1458: F, t2039: F, t24972: F, t27921: F, t32406: F, t33192: F, t33195: F, t33641: F, t33643: F, t33645: F, t33653: F, t33655: F, t33658: F, t33661: F, t577: F, t7423: F, t7801: F, t7956: F, t8508: F) -> (F, F, F) {
    let t34175 = t34104 + t34115 + t34157 + t34173;
    let t34176 = t3 * t34175;
    let t34194 = F::new(0.45e1) * t34175 * t577 + F::new(0.135e2) * t32406 * t1458 + F::new(0.135e2) * t27921 * t2039 + F::new(27.0) * t24972 * t7956 + F::new(0.135e2) * t7423 * t7801 + t33641 + t33643 + t33645 + t33653 + t33655 + t33658 + t33661 + t33192 + t33195 + t8508;
    (t34175, t34176, t34194)
}
