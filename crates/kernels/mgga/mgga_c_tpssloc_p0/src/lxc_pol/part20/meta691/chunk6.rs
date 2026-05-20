//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2629/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2629<F: Float>(t11761: F, t1232: F, t14725: F, t3577: F, t45128: F, t45256: F, t45260: F, t45262: F, t4889: F, t52538: F, t53481: F, t53487: F, t53490: F, t53494: F, t53496: F, t53498: F) -> F {
    let t53503 = -t53481 / F::new(576.0) - F::new(5.0) / F::new(1728.0) * t3577 * t45128 * t14725 * t52538 - t53487 * t1232 / F::new(1536.0) - F::new(5.0) / F::new(486.0) * t53490 - F::new(2.0) / F::new(27.0) * t4889 * t11761 - t53494 / F::new(1152.0) + t53496 / F::new(108.0) + t53498 / F::new(54.0) + F::new(5.0) / F::new(6912.0) * t45256 + F::new(5.0) / F::new(3456.0) * t45260 + t45262 / F::new(1536.0);
    t53503
}
