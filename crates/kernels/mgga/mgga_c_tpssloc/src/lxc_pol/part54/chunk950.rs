//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 950/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk950<F: Float>(t240: F, t67: F, t1864: F, t1860: F, t6509: F, t7031: F, t22819: F, t22825: F, t22858: F, t22863: F, t22867: F, t22645: F) -> (F, F, F, F, F, F, F, F) {
    let t23992 = t240 * t67;
    let t23993 = t23992 * t1864;
    let t23995 = F::new(88.0) / F::new(27.0) * t1860 * t23993;
    let t23998 = t7031 * t6509;
    let t23999 = t1860 * t23998;
    let t24049 = F::new(0.33643963411783659044e-4) * t22819;
    let t24050 = F::new(0.10541775202358879834e-2) * t22825;
    let t24058 = F::new(119.0) / F::new(3456.0) * t22858;
    let t24060 = F::new(35.0) / F::new(216.0) * t22863;
    let t24061 = F::new(0.22608743412718618878e-1) * t22867;
    let t24071 = F::new(0.16449340668482264365e-1) * t22645;
    (t23995, t23999, t24049, t24050, t24058, t24060, t24061, t24071)
}
