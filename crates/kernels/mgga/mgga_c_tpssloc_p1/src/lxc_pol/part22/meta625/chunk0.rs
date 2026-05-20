//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2159/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2159<F: Float>(t53782: F, t16169: F, t2663: F, t15892: F, t2371: F, t5154: F, t9919: F, t12344: F, t5234: F, t1369: F, t1831: F, t40059: F) -> (F, F, F, F, F, F, F) {
    let t53783 = F::cast_from(0.32530743900905219526e-1_f64) * t53782;
    let t53787 = t16169 * t2663;
    let t53788 = F::cast_from(0.73245789224026180216e-3_f64) * t53787;
    let t53796 = t15892 * t2371;
    let t53797 = F::cast_from(0.35089341735807877242e1_f64) * t53796;
    let t53798 = t5154 * t9919;
    let t53880 = t5234 * t12344;
    let t53881 = t53880 * t1369;
    let t53882 = F::new(119.0) / F::new(1152.0) * t53881;
    let t53901 = t40059 * t1831;
    (t53783, t53788, t53797, t53798, t53880, t53882, t53901)
}
