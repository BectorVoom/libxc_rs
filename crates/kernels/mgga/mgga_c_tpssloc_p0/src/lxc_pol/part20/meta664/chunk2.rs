//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2487/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2487<F: Float>(t43835: F, t43837: F, t43839: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t50881: F, t50886: F, t50897: F, t50900: F) -> F {
    let t50902 = F::new(0.198684e1) * t50881 - F::new(0.82785e-1) * t50886 + F::new(0.11038e0) * t43835 - F::new(0.33114e0) * t43837 - F::new(0.5519e-1) * t43839 - F::cast_from(0.91983333333333333335e-1_f64) * t43855 - F::cast_from(0.24528888888888888889e-1_f64) * t43857 - F::cast_from(0.73586666666666666668e0_f64) * t43859 + F::new(0.27595e0) * t43861 + F::cast_from(0.55190000000000000001e0_f64) * t43863 - F::cast_from(0.20128333333333333333e0_f64) * t50897 - F::new(0.72462e1) * t50900;
    t50902
}
