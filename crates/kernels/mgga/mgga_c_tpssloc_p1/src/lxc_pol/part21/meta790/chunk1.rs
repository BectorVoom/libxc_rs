//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2750/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2750<F: Float>(t2659: F, t57973: F, t16606: F, t2379: F, t39463: F, t39468: F, t40714: F, t40716: F, t4314: F, t57959: F, t57961: F, t57962: F, t57966: F, t57970: F, t57972: F) -> (F, F) {
    let t57975 = F::new(12.0) * t57973 * t2659;
    let t57976 = F::new(6.0) * t16606 * t2379 * t4314 + t39463 - t39468 - t40714 + t40716 + t57959 + t57961 - t57962 + t57966 + t57970 + t57972 + t57975;
    (t57975, t57976)
}
