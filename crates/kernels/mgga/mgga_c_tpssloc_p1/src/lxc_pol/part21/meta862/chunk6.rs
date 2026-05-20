//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3135/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3135<F: Float>(t1174: F, t6140: F, t698: F, t63841: F, t63843: F, t63845: F, t63886: F, t63888: F, t63891: F, t63893: F, t63896: F, t63899: F, t63903: F, t63906: F, t63909: F) -> (F, F) {
    let t64885 = t1174 * t698 * t6140;
    let t64903 = F::new(8.0) / F::new(81.0) * t63841 + F::new(4.0) / F::new(9.0) * t63843 - F::new(2.0) / F::new(27.0) * t63845 + F::new(2.0) / F::new(9.0) * t63886 + F::new(5.0) / F::new(81.0) * t63888 + t63891 / F::new(9.0) - F::new(10.0) / F::new(27.0) * t63893 - F::new(2.0) / F::new(3.0) * t63896 - F::new(8.0) / F::new(27.0) * t63899 - F::new(2.0) / F::new(3.0) * t63903 - t63906 / F::new(3.0) - t63909;
    (t64885, t64903)
}
