//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1482/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1482<F: Float>(t1229: F, t204: F, t1090: F, t1227: F, t248: F, t11692: F, t1174: F, t1177: F, t11779: F, t11781: F, t11825: F, t1213: F, t1214: F, t1216: F, t3490: F, t3515: F, t3527: F, t3578: F, t3585: F, t43719: F, t43752: F, t43792: F, t43796: F, t44668: F, t44798: F, t45250: F, t45251: F, t45256: F, t45260: F, t45262: F, t45264: F, t45266: F, t45271: F, t45283: F, t475: F) -> F {
    let t45293 = t204 * t1229;
    let t45296 = t1227 * t248 * t45293 * t1090;
    let t45311 = -t45250 + t11692 * t3578 * t1216 * t45251 / F::new(384.0) + F::new(5.0) / F::new(1728.0) * t45256 + F::new(5.0) / F::new(864.0) * t45260 + t45262 / F::new(384.0) - t45264 / F::new(576.0) - t45266 / F::new(1152.0) - F::new(5.0) / F::new(1944.0) * t45271 + F::new(5.0) / F::new(4608.0) * t1227 * t248 * t3585 * t43752 + F::new(5.0) / F::new(384.0) * t1227 * t248 * t3585 * t43796 - t45283 / F::new(192.0) - t1174 * t1177 * t43719 / F::new(8.0) - t3515 * t248 * t1214 * t44668 * t475 / F::new(1024.0) - t45296 / F::new(3888.0) - F::new(5.0) / F::new(1296.0) * t3490 * t11781 - F::new(5.0) / F::new(432.0) * t1227 * t248 * t11779 * t43792 + t1213 * t248 * t1214 * t44798 * t475 / F::new(3072.0) - t11825 * t3527 / F::new(768.0);
    t45311
}
