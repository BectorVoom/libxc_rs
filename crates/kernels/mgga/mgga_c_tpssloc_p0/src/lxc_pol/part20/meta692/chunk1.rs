//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2636/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2636<F: Float>(t11605: F, t1760: F, t11868: F, t1190: F, t11918: F, t11919: F, t11928: F, t11934: F, t1238: F, t14972: F, t15771: F, t15787: F, t15790: F, t1720: F, t1761: F, t27784: F, t3487: F, t3590: F, t3593: F, t3598: F, t3631: F, t45345: F, t45355: F, t45375: F, t4940: F, t498: F, t5055: F, t5089: F) -> F {
    let t53677 = t11605 * t1760;
    let t53697 = F::new(2.0) * t11918 * t1238 * t1760 * t3598 + t11868 * t1720 * t498 + F::new(3.0) * t1190 * t15771 * t498 - F::new(18.0) * t11934 * t27784 * t53677 + F::new(3.0) * t3590 * t4940 * t498 - t11919 * t5055 - F::new(3.0) * t11928 * t5089 - F::new(3.0) * t14972 * t3631 - F::new(3.0) * t15787 * t3593 + F::new(12.0) * t15790 * t3487 - F::new(3.0) * t1761 * t45345 - F::new(3.0) * t1761 * t45355 - t1761 * t45375;
    t53697
}
