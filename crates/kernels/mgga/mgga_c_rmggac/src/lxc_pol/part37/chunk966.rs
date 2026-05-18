//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 966/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk966<F: Float>(t77473: F, t69586: F, t71380: F, t75137: F, t75139: F, t75145: F, t75149: F, t75152: F, t75157: F, t75163: F, t75186: F, t75192: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77474 = F::new(0.36021158228745895953e-3) * t77473;
    let t77476 = F::new(0.20496175532535769483e-3) * t69586;
    let t77477 = F::new(0.15243824895787514157e-3) * t71380;
    let t77480 = F::new(0.19709219354514038085e-5) * t75137;
    let t77481 = F::new(0.638468998399467591e-4) * t75139;
    let t77484 = F::new(0.1276937996798935182e-4) * t75145;
    let t77485 = F::new(0.1276937996798935182e-4) * t75149;
    let t77486 = F::new(0.1276937996798935182e-4) * t75152;
    let t77487 = F::new(0.16360768083986357019e-4) * t75157;
    let t77491 = F::new(0.44903406381989282115e-1) * t75163;
    let t77502 = F::new(0.85129199786595678799e-5) * t75186;
    let t77503 = F::new(0.85129199786595678799e-5) * t75192;
    (t77474, t77476, t77477, t77480, t77481, t77484, t77485, t77486, t77487, t77491, t77502, t77503)
}
