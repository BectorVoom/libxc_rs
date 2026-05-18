//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1017/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1017<F: Float>(t23095: F, t23105: F, t23107: F, t23140: F, t23143: F, t23013: F, t23031: F, t2047: F, t2627: F, t23173: F, t23230: F, t23251: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t24218 = F::new(0.10541775202358879834e-2) * t23095;
    let t24220 = F::new(0.33643963411783659044e-4) * t23105;
    let t24221 = F::new(119.0) / F::new(3456.0) * t23107;
    let t24230 = F::new(0.22608743412718618878e-1) * t23140;
    let t24231 = F::new(35.0) / F::new(216.0) * t23143;
    let t24246 = F::new(0.12793931631041761173e0) * t23013;
    let t24250 = F::new(0.52089578783527170489e-1) * t23031;
    let t24255 = t2627 * t2047;
    let t24265 = F::new(0.16449340668482264365e-1) * t23173;
    let t24291 = F::new(0.16449340668482264365e-1) * t23230;
    let t24318 = F::new(0.52089578783527170489e-1) * t23251;
    (t24218, t24220, t24221, t24230, t24231, t24246, t24250, t24255, t24265, t24291, t24318)
}
