//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 575/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk575<F: Float>(t1877: F, t1915: F, t25: F, t2522: F, t606: F, t6542: F, t6666: F, t6670: F, t6671: F, t337: F, t614: F, t1887: F) -> (F, F, F) {
    let t6678 = F::new(3.0) / F::new(2.0) * t2522 * t1915 * t6542 + t1877 * t6666 * t25 / F::new(2.0) - t1877 * t6670 * t6671 / F::new(2.0) + t1877 * t1915 * t606 / F::new(2.0);
    let t6679 = t614 * t337;
    let t6680 = t6679 * t1887;
    (t6678, t6679, t6680)
}
