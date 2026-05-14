//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 848/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk848<F: Float>(t78093: F, t69437: F, t69445: F, t25820: F, t77091: F, t27048: F, t77338: F, t76363: F, t76365: F, t2471: F, t265: F, t305: F, t76373: F, t76375: F, t76377: F, t76379: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t78094 = 0.6818665413561335432e-1 * t78093;
    let t78098 = 0.21819729323396273382e0 * t69437;
    let t78099 = 0.54549323308490683456e-1 * t69445;
    let t78100 = t25820 * t77091;
    let t78101 = 0.8980681276397856423e-1 * t78100;
    let t78103 = 0.35922725105591425692e0 * t27048 * t77338;
    let t78110 = 0.10909864661698136691e0 * t76363;
    let t78111 = 0.21819729323396273382e0 * t76365;
    let t78112 = t2471 * t265;
    let t78113 = t305 * t78112;
    let t78114 = 0.39914139006212695213e-1 * t78113;
    let t78115 = 0.20455996240684006298e-1 * t76373;
    let t78116 = 0.20455996240684006298e-1 * t76375;
    let t78117 = 0.2727466165424534173e-1 * t76377;
    let t78119 = 0.2727466165424534173e-1 * t76379;
    (t78094, t78098, t78099, t78101, t78103, t78110, t78111, t78112, t78114, t78115, t78116, t78117, t78119)
}
