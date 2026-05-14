//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 651/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk651<F: Float>(t71372: F, t14417: F, t2046: F, t7297: F, t2039: F, t2244: F, t270: F, t638: F, t2227: F, t235: F, t7190: F, t7262: F, t14696: F, t7491: F, t69674: F, t1341: F, t703: F, t7310: F) -> (F, F, F, F, F, F, F, F) {
    let t71373 = 0.81300399444200075504e-3 * t71372;
    let t71375 = t2046 * t7297 * t14417;
    let t71376 = 0.1951603679568577289e-3 * t71375;
    let t71380 = t638 * t2039 * t2244 * t270;
    let t71400 = t235 * t7190 * t2227;
    let t71404 = t235 * t7262 * t2227;
    let t71418 = t7491 * t14696;
    let t71419 = 0.30487649791575028314e-3 * t71418;
    let t71429 = 0.11351689503877428609e-7 * t69674;
    let t71446 = t638 * t7310 * t703 * t1341;
    (t71373, t71376, t71380, t71400, t71404, t71419, t71429, t71446)
}
