//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1176/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1176<F: Float>(t25: F, t868: F, t1877: F, t1915: F, t2522: F, t606: F, t6542: F, t6666: F, t6670: F, t337: F, t614: F, t1887: F) -> (F, F, F, F) {
    let t6671 = t25 * t868;
    let t6678 = F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t6542 + t1877 * t6666 * t25 / F::cast_from(2.0_f64) - t1877 * t6670 * t6671 / F::cast_from(2.0_f64) + t1877 * t1915 * t606 / F::cast_from(2.0_f64);
    let t6679 = t614 * t337;
    let t6680 = t6679 * t1887;
    (t6671, t6678, t6679, t6680)
}
