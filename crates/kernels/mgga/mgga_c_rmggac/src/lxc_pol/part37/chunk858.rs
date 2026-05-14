//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 858/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk858<F: Float>(t739: F, t78112: F, t75972: F, t75978: F, t15464: F, t5016: F, t9128: F, t70149: F, t70156: F, t71717: F, t71720: F, t70169: F, t70173: F, t70176: F, t70208: F, t1356: F, t78022: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t78390 = t739 * t78112;
    let t78391 = 0.39914139006212695213e-1 * t78390;
    let t78394 = 0.1702583995731913576e-4 * t75972;
    let t78395 = 0.11634323970834742769e-4 * t75978;
    let t78396 = t5016 * t15464;
    let t78397 = 0.2993560425465952141e-1 * t78396;
    let t78398 = t9128 * t15464;
    let t78399 = 0.2993560425465952141e-1 * t78398;
    let t78400 = 0.54549323308490683456e-1 * t70149;
    let t78401 = 0.21819729323396273382e0 * t70156;
    let t78402 = 0.40650199722100037752e-3 * t71717;
    let t78403 = 0.40650199722100037752e-3 * t71720;
    let t78404 = 0.72042316457491791901e-3 * t70169;
    let t78405 = 0.38430329123504567781e-4 * t70173;
    let t78406 = 0.638468998399467591e-4 * t70176;
    let t78409 = 0.79808624799933448875e-4 * t70208;
    let t78423 = 0.39914139006212695214e-1 * t1356 * t78022;
    (t78391, t78394, t78395, t78397, t78399, t78400, t78401, t78402, t78403, t78404, t78405, t78406, t78409, t78423)
}
