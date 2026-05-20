//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2114;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta677<F: Float>(t24740: F, t5064: F, t15640: F, t24729: F, t24574: F, t27574: F, t24844: F, t7999: F, t2121: F, t3427: F, t8077: F, t27517: F, t85639: F, t27481: F, t7365: F, t94490: F, t1715: F, t974: F, t24847: F, t24771: F, t15418: F, t2127: F, t221: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t95687, t95702, t95714, t95722, t95726, t95747) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2114::<F>(t24740, t5064, t15640, t24729, t24574, t27574, t24844, t7999, t2121, t3427, t8077, t27517, t85639);
        let (t95751, t95758, t95760, t95761, t95768, t95772) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2115::<F>(t24574, t27481, t7365, t94490, t1715, t974, t24847, t24771, t7999, t15418, t2127, t221);
    (t95687, t95702, t95714, t95722, t95726, t95747, t95751, t95758, t95760, t95761, t95768, t95772)
}
