//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2702/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2702<F: Float>(t12466: F, t12477: F, t1297: F, t1390: F, t193: F, t39249: F, t39256: F, t39261: F, t39266: F, t39304: F, t39309: F, t39312: F, t39316: F, t5126: F, t5161: F, t5308: F, t533: F, t53778: F, t53780: F, t53783: F, t53788: F, t53789: F, t53797: F, t53799: F, t53800: F, t53856: F, t54832: F, t55088: F, t55124: F, t55155: F) -> F {
    let t55161 = -t53778 - t53780 + t53783 - F::new(18.0) * t5126 * t12477 * t5308 + t53788 - t39249 - F::new(18.0) * t5126 * t5161 * t53789 + F::new(18.0) * t5126 * t12466 * t5308 - t39256 + t53797 - t53799 - t39261 - t39266 - t39304 + t53800 + F::new(3.0) * t193 * t1297 * t53856 + t193 * t533 * (t54832 + t55088 + t55124 + t55155) * t1390 - t39309 + t39312 + t39316;
    t55161
}
