//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 714/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk714<F: Float>(t33: F, t625: F, t2240: F, t240: F, t67: F, t1864: F, t1860: F, t22819: F, t22825: F, t22858: F, t22863: F, t22867: F) -> (F, F, F, F, F, F, F, F) {
    let t23966 = t33 * t625;
    let t23967 = t2240 * t23966;
    let t23992 = t240 * t67;
    let t23993 = t23992 * t1864;
    let t23995 = F::new(88.0) / F::new(27.0) * t1860 * t23993;
    let t24049 = F::cast_from(0.33643963411783659044e-4_f64) * t22819;
    let t24050 = F::cast_from(0.10541775202358879834e-2_f64) * t22825;
    let t24058 = F::new(119.0) / F::new(3456.0) * t22858;
    let t24060 = F::new(35.0) / F::new(216.0) * t22863;
    let t24061 = F::cast_from(0.22608743412718618878e-1_f64) * t22867;
    (t23966, t23967, t23995, t24049, t24050, t24058, t24060, t24061)
}
