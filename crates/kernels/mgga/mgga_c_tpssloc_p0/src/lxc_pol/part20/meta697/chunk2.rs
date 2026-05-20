//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2663/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2663<F: Float>(t28: F, t5145: F, t591: F, t1081: F, t11122: F, t12001: F, t12072: F, t15952: F, t15955: F, t16: F, t1649: F, t2: F, t3672: F, t39436: F, t5142: F, t517: F, t53832: F, t53835: F, t53841: F, t53844: F, t584: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t54370 = F::new(32.0) * t5145 * t591;
    let t54372 = piecewise3::<F>(t29, F::new(0.0), F::new(40.0) / F::new(81.0) * t39436 * t1649 * t12001 + F::new(16.0) / F::new(9.0) * t12072 * t2 * t53832 - F::new(8.0) / F::new(9.0) * t15952 * t53835 - F::new(8.0) / F::new(3.0) * t3672 * t584 * t1081 + F::new(8.0) * t15955 * t53841 - F::new(8.0) / F::new(3.0) * t15955 * t53844 + F::new(4.0) / F::new(9.0) * t5142 * t11122 + F::new(16.0) * t517 * t16 - t54370);
    t54372
}
