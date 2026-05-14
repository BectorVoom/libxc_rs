//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 827/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk827<F: Float>(t69728: F, t14438: F, t2868: F, t14498: F, t5928: F, t15526: F, t2604: F, t69745: F, t1550: F, t699: F, t8708: F, t75443: F, t15450: F, t7255: F, t1970: F, t1971: F, t209: F, t2227: F, t515: F, t605: F) -> (F, F, F, F, F, F, F, F, F) {
    let t77591 = 0.30487649791575028312e-3 * t69728;
    let t77592 = t2868 * t14438;
    let t77593 = 0.14967802127329760705e-1 * t77592;
    let t77595 = 0.39914139006212695214e-1 * t5928 * t14498;
    let t77596 = t2604 * t15526;
    let t77597 = 0.14967802127329760705e-1 * t77596;
    let t77598 = 0.16263363996404810741e-4 * t69745;
    let t77604 = t1550 * t699 * t8708;
    let t77605 = 0.79828278012425390427e-1 * t77604;
    let t77606 = 0.54549323308490683456e-1 * t75443;
    let t77607 = t7255 * t15450;
    let t77608 = 0.42564599893297839398e-5 * t77607;
    let t77613 = t1970 * t1971 * t515 * t2227 * t605 * t209;
    (t77591, t77593, t77595, t77597, t77598, t77605, t77606, t77608, t77613)
}
