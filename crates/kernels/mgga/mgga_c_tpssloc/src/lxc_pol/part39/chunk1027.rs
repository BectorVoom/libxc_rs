//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1027/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1027<F: Float>(t1041: F, t14202: F, t1009: F, t4552: F, t1011: F, t1019: F, t1615: F, t3131: F, t1022: F, t883: F, t607: F, t3071: F, t360: F, t4342: F, t1025: F, t10403: F, t10413: F, t10909: F, t10923: F, t10927: F, t14174: F, t14180: F, t14184: F, t14189: F, t14194: F, t14198: F, t2960: F, t3070: F, t3117: F, t4590: F, t4609: F, t973: F) -> (F, F, F) {
    let t14203 = t1041 * t14202;
    let t14205 = t4552 * t1009;
    let t14206 = t14205 * t1011;
    let t14207 = t14206 * t1019;
    let t14211 = t1615 * t3131;
    let t14212 = t1022 * t883;
    let t14213 = t14212 * t607;
    let t14214 = t14211 * t14213;
    let t14215 = t3071 * t14214;
    let t14218 = t1615 * t1022;
    let t14219 = t360 * t883;
    let t14220 = t14219 * t607;
    let t14221 = t14218 * t14220;
    let t14222 = t3071 * t14221;
    let t14227 = t607 * t1022;
    let t14228 = t14227 * t360;
    let t14229 = t4342 * t14228;
    let t14230 = t3071 * t14229;
    let t14233 = -5.0 / 2304.0 * t1041 * t14174 + 5.0 / 6912.0 * t3117 * t4590 + 5.0 / 6912.0 * t1041 * t14180 + 5.0 / 13824.0 * t1041 * t14184 + 5.0 / 5184.0 * t1041 * t14189 + t14194 - t2960 * t4609 / 54.0 + t973 * t14198 / 288.0 - t14203 / 20736.0 + t14207 * t1025 / 1536.0 + t10909 / 4608.0 + t10403 * t14215 / 1152.0 - t10413 * t14222 / 2304.0 - t10923 / 648.0 - t10927 / 162.0 - t3070 * t14230 / 1152.0;
    (t14205, t14228, t14233)
}
