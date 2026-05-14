//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1343/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1343<F: Float>(t11820: F, t3536: F, t11778: F, t121: F, t11148: F, t1227: F, t248: F, t11728: F, t11729: F, t3570: F, t1229: F, t204: F, t1090: F, t11692: F, t1174: F, t1177: F, t11779: F, t11781: F, t11825: F, t1213: F, t1214: F, t1216: F, t3490: F, t3515: F, t3527: F, t3578: F, t3585: F, t43719: F, t43752: F, t43792: F, t43796: F, t44668: F, t44798: F, t45250: F, t45251: F, t45256: F, t45260: F, t45262: F, t45264: F, t475: F) -> (F,) {
    let t45266 = t3536 * t11820;
    let t45268 = t121 * t11778;
    let t45271 = t1227 * t248 * t45268 * t11148;
    let t45283 = t11728 * t248 * t3570 * t11729;
    let t45293 = t204 * t1229;
    let t45296 = t1227 * t248 * t45293 * t1090;
    let t45311 = -t45250 + t11692 * t3578 * t1216 * t45251 / 384.0 + 5.0 / 1728.0 * t45256 + 5.0 / 864.0 * t45260 + t45262 / 384.0 - t45264 / 576.0 - t45266 / 1152.0 - 5.0 / 1944.0 * t45271 + 5.0 / 4608.0 * t1227 * t248 * t3585 * t43752 + 5.0 / 384.0 * t1227 * t248 * t3585 * t43796 - t45283 / 192.0 - t1174 * t1177 * t43719 / 8.0 - t3515 * t248 * t1214 * t44668 * t475 / 1024.0 - t45296 / 3888.0 - 5.0 / 1296.0 * t3490 * t11781 - 5.0 / 432.0 * t1227 * t248 * t11779 * t43792 + t1213 * t248 * t1214 * t44798 * t475 / 3072.0 - t11825 * t3527 / 768.0;
    (t45311,)
}
