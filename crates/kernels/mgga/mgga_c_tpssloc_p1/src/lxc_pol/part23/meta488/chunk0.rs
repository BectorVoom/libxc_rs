//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1494/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1494<F: Float>(t54389: F, t56185: F, t54392: F, t74072: F, t74074: F, t74077: F, t54411: F, t54412: F, t20416: F, t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t39483: F, t5126: F, t5127: F) -> (F, F, F, F, F, F, F, F, F) {
    let t79904 = F::cast_from(0.23392894490538584828e1_f64) * t54389;
    let t79905 = F::cast_from(48.0_f64) * t56185;
    let t79906 = F::cast_from(0.14035736694323150897e2_f64) * t54392;
    let t79907 = F::cast_from(16.0_f64) * t74072;
    let t79908 = F::cast_from(16.0_f64) * t74074;
    let t79909 = F::cast_from(0.23392894490538584828e1_f64) * t74077;
    let t79910 = F::cast_from(4.0_f64) * t54411;
    let t79914 = F::cast_from(48.0_f64) * t54412;
    let t79915 = F::cast_from(24.0_f64) * t20416 * t5126 * t5127 + t39411 + t39463 - t39468 - t39472 - t39476 + t39483 - t79904 - t79905 + t79906 - t79907 - t79908 - t79909 + t79910 - t79914;
    (t79904, t79905, t79906, t79907, t79908, t79909, t79910, t79914, t79915)
}
