//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2629/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2629<F: Float>(t11761: F, t1232: F, t14725: F, t3577: F, t45128: F, t45256: F, t45260: F, t45262: F, t4889: F, t52538: F, t53481: F, t53487: F, t53490: F, t53494: F, t53496: F, t53498: F) -> F {
    let t53503 = -t53481 / F::cast_from(576.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t3577 * t45128 * t14725 * t52538 - t53487 * t1232 / F::cast_from(1536.0_f64) - F::cast_from(5.0_f64) / F::cast_from(486.0_f64) * t53490 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t4889 * t11761 - t53494 / F::cast_from(1152.0_f64) + t53496 / F::cast_from(108.0_f64) + t53498 / F::cast_from(54.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t45256 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t45260 + t45262 / F::cast_from(1536.0_f64);
    t53503
}
