//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 933/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk933<F: Float>(t76017: F, t15001: F, t551: F, t70230: F, t71744: F, t73480: F, t739: F, t75993: F, t75997: F, t76000: F, t76002: F, t76021: F, t76025: F, t78423: F, t78427: F, t78431: F, t78434: F, t78436: F, t78438: F) -> (F, F) {
    let t80395 = 0.29085809927086856922e-4 * t76017;
    let t80398 = t15001 * t551;
    let t80401 = -0.58171619854173713844e-5 * t75993 + 0.58171619854173713844e-5 * t75997 - 0.31062809106223861414e-2 * t76000 + t76002 + t78423 - t70230 - t80395 + t71744 + t78427 + 0.76860658247009135562e-5 * t76021 - t78431 - t78434 - 0.40878380883436523435e-5 * t76025 - 0.59871208509319042821e-1 * t739 * t80398 + t78436 + t73480 + t78438;
    (t80398, t80401)
}
