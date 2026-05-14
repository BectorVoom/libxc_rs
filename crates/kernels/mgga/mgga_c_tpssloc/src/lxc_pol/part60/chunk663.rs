//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 663/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk663<F: Float>(t23095: F, t23105: F, t23107: F, t23140: F, t23143: F, t23013: F, t23031: F, t2047: F, t2627: F, t23173: F, t23230: F, t23251: F, t23261: F, t10143: F, t2056: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24218 = 0.10541775202358879834e-2 * t23095;
    let t24220 = 0.33643963411783659044e-4 * t23105;
    let t24221 = 119.0 / 3456.0 * t23107;
    let t24230 = 0.22608743412718618878e-1 * t23140;
    let t24231 = 35.0 / 216.0 * t23143;
    let t24246 = 0.12793931631041761173e0 * t23013;
    let t24250 = 0.52089578783527170489e-1 * t23031;
    let t24255 = t2627 * t2047;
    let t24265 = 0.16449340668482264365e-1 * t23173;
    let t24291 = 0.16449340668482264365e-1 * t23230;
    let t24318 = 0.52089578783527170489e-1 * t23251;
    let t24321 = 0.12793931631041761173e0 * t23261;
    let t24344 = t2056 * t10143;
    (t24218, t24220, t24221, t24230, t24231, t24246, t24250, t24255, t24265, t24291, t24318, t24321, t24344)
}
