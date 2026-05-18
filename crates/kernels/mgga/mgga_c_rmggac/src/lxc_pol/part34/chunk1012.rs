//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1012/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1012<F: Float>(t75473: F, t75477: F, t75480: F, t75484: F, t15597: F, t874: F, t352: F, t75508: F, t75513: F, t75517: F, t75522: F, t1356: F, t69827: F, t71502: F, t71505: F, t75490: F, t75495: F, t75500: F, t75519: F, t75524: F) -> (F, F) {
    let t77633 = F::new(0.7661627980793611092e-4) * t75473;
    let t77634 = F::new(0.5107751987195740728e-4) * t75477;
    let t77635 = F::new(0.2553875993597870364e-4) * t75480;
    let t77636 = F::new(0.43368970657079495308e-4) * t75484;
    let t77637 = t874 * t15597;
    let t77638 = t77637 * t352;
    let t77641 = F::new(0.86737941314158990619e-4) * t75508;
    let t77642 = F::new(0.81300399444200075499e-3) * t75513;
    let t77643 = F::new(0.54549323308490683461e-1) * t75517;
    let t77646 = F::new(0.9197635698773217773e-5) * t75522;
    let t77648 = t77633 + t77634 - t77635 + t77636 - t71502 + F::new(0.39914139006212695214e-1) * t1356 * t77638 - t75490 - t75495 + t75500 + t77641 - t77642 + t77643 - F::new(0.24527028530061914063e-5) * t75519 - F::new(0.29085809927086856923e-4) * t69827 - t77646 - F::new(0.40878380883436523436e-5) * t75524 + t71505;
    (t77638, t77648)
}
