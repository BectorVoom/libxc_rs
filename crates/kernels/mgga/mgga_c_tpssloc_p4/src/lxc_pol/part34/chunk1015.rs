//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1015/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1015<F: Float>(t1864: F, t23992: F, t1860: F, t22819: F, t22825: F, t22858: F, t22863: F, t22867: F, t22645: F, t22692: F, t22717: F, t22725: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t23993 = t23992 * t1864;
    let t23995 = F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t1860 * t23993;
    let t24049 = F::cast_from(0.33643963411783659044e-4_f64) * t22819;
    let t24050 = F::cast_from(0.10541775202358879834e-2_f64) * t22825;
    let t24058 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t22858;
    let t24060 = F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t22863;
    let t24061 = F::cast_from(0.22608743412718618878e-1_f64) * t22867;
    let t24071 = F::cast_from(0.16449340668482264365e-1_f64) * t22645;
    let t24099 = F::cast_from(0.16449340668482264365e-1_f64) * t22692;
    let t24108 = F::cast_from(0.12793931631041761173e0_f64) * t22717;
    let t24110 = F::cast_from(0.52089578783527170489e-1_f64) * t22725;
    (t23993, t23995, t24049, t24050, t24058, t24060, t24061, t24071, t24099, t24108, t24110)
}
