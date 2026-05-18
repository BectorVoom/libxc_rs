//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1073/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1073<F: Float>(t1356: F, t78022: F, t77980: F, t2392: F, t739: F, t8264: F, t2211: F, t8924: F, t76027: F, t70212: F, t70229: F, t71744: F, t71755: F, t75993: F, t75997: F, t76000: F, t76002: F, t76017: F, t76021: F, t76025: F, t78087: F) -> F {
    let t78423 = F::new(0.39914139006212695214e-1) * t1356 * t78022;
    let t78427 = F::new(0.39914139006212695214e-1) * t1356 * t77980;
    let t78430 = t739 * t8264 * t2392;
    let t78431 = F::new(0.2993560425465952141e-1) * t78430;
    let t78433 = t739 * t2211 * t8924;
    let t78434 = F::new(0.2993560425465952141e-1) * t78433;
    let t78436 = F::new(0.38430329123504567781e-4) * t76027;
    let t78437 = t70212 - F::new(0.58171619854173713846e-5) * t75993 + F::new(0.58171619854173713846e-5) * t75997 - F::new(0.31062809106223861415e-2) * t76000 - F::new(0.59871208509319042821e-1) * t739 * t78087 + t76002 + t78423 - F::new(0.12263514265030957031e-4) * t70229 - F::new(0.29085809927086856923e-4) * t76017 + t71744 + t78427 + F::new(0.76860658247009135557e-5) * t76021 - t78431 - t78434 - F::new(0.40878380883436523436e-5) * t76025 + t78436 + t71755;
    t78437
}
