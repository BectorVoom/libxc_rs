//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3072/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3072<F: Float>(t1671: F, t51397: F, t18786: F, t3371: F, t63717: F, t63720: F, t63722: F, t63725: F, t63729: F, t63731: F, t63733: F, t63735: F, t63737: F, t63739: F, t63741: F, t63743: F, t63745: F, t63747: F, t63752: F, t63754: F, t63757: F) -> (F, F) {
    let t63759 = F::new(2.0) * t51397 * t1671;
    let t63760 = -t63717 - t63720 - t63722 - t63725 - t63729 + t63731 + t63733 + t63735 + t63737 - t63739 - t63741 - t63743 - t63745 - t63747 + F::cast_from(0.11696447245269292414e1_f64) * t3371 * t18786 - t63752 - t63754 - t63757 - t63759;
    (t63759, t63760)
}
