//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2289/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2289<F: Float>(t80711: F, t22724: F, t26474: F, t22751: F, t26194: F, t1887: F, t80830: F, t22635: F, t26332: F, t3734: F, t22916: F, t26193: F, t6888: F) -> (F, F, F, F, F, F) {
    let t90581 = F::cast_from(0.52089578783527170489e-1_f64) * t80711;
    let t90582 = t22724 * t26474;
    let t90584 = t22751 * t26194;
    let t90585 = F::cast_from(0.76763589786250567036e-1_f64) * t90584;
    let t90591 = t80830 * t1887;
    let t90594 = t90591 * t22635 * t26332 * t3734;
    let t90598 = t6888 * t26193 * t22916;
    (t90581, t90582, t90585, t90591, t90594, t90598)
}
