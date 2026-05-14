//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 661/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk661<F: Float>(t240: F, t67: F, t1864: F, t1860: F, t22819: F, t22825: F, t22858: F, t22863: F, t22867: F, t22645: F, t22692: F, t22717: F, t22725: F, t2085: F, t3787: F, t22923: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23992 = t240 * t67;
    let t23993 = t23992 * t1864;
    let t23995 = 88.0 / 27.0 * t1860 * t23993;
    let t24049 = 0.33643963411783659044e-4 * t22819;
    let t24050 = 0.10541775202358879834e-2 * t22825;
    let t24058 = 119.0 / 3456.0 * t22858;
    let t24060 = 35.0 / 216.0 * t22863;
    let t24061 = 0.22608743412718618878e-1 * t22867;
    let t24071 = 0.16449340668482264365e-1 * t22645;
    let t24099 = 0.16449340668482264365e-1 * t22692;
    let t24108 = 0.12793931631041761173e0 * t22717;
    let t24110 = 0.52089578783527170489e-1 * t22725;
    let t24127 = t3787 * t2085;
    let t24156 = 0.12793931631041761173e0 * t22923;
    (t23995, t24049, t24050, t24058, t24060, t24061, t24071, t24099, t24108, t24110, t24127, t24156)
}
