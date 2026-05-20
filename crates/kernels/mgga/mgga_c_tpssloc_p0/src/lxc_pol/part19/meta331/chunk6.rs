//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1187/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1187<F: Float>(t12019: F, t566: F, t68: F, t3888: F, t12023: F, t12027: F, t12030: F, t12033: F, t12181: F, t12237: F, t12238: F, t12240: F, t12249: F, t12251: F, t12252: F, t12259: F, t12267: F, t12434: F, t12438: F, t1323: F, t1336: F, t1352: F, t1372: F, t1375: F, t1378: F, t1380: F, t1381: F, t1383: F, t22694: F, t22740: F, t3752: F, t3758: F, t3777: F, t3793: F, t3851: F, t3879: F, t3882: F, t3889: F, t3897: F, t3898: F, t3902: F, t3907: F, t39938: F, t40047: F, t40118: F, t40133: F, t40148: F, t40153: F, t40162: F, t40438: F, t40453: F, t40475: F, t40479: F, t40486: F, t40492: F, t40524: F, t40541: F, t40576: F, t5250: F, t5334: F, t5344: F, t539: F, t562: F, t568: F) -> F {
    let t40590 = F::new(1.0) / t12019 / t566;
    let t40591 = t68 * t40590;
    let t40592 = t3888 * t3888;
    let t40603 = t539 * t40453 * t568 + F::new(12.0) * t12030 * t3889 + F::new(12.0) * t12033 * t3889 - F::new(24.0) * t3758 * t12023 + F::new(24.0) * t3882 * t12027 - t1375 * t1378 * (-F::new(24.0) * t1336 * t40492 * t12251 - F::new(6.0) * t1336 * t12259 * t3851 - F::new(4.0) * t1336 * t40479 * t1352 - F::new(3.0) * t1336 * t1380 * t39938 + F::new(12.0) * t1336 * t40486 * t3793 + F::new(6.0) * t1336 * t3897 * t40133 - F::new(6.0) * t5344 * t22740 * t3851 + F::new(8.0) * t5334 * t40475 * t5250 - F::new(24.0) * t3777 * t12252 - F::new(12.0) * t12267 * t3902 + t40524 + F::new(24.0) * t5334 * t22694 * t12240 - F::new(36.0) * t1336 * t12249 * t40148 - t1336 * t1380 * t40153 + F::new(14.0) * t1336 * t3897 * t40162 + F::new(24.0) * t1336 * t40541 * t40047 - F::new(12.0) * t3777 * t12181 + F::new(4.0) * t12238 * t1383 + F::new(12.0) * t12267 * t3898 - F::new(6.0) * t12267 * t3907 - F::new(4.0) * t40118 * t1381 + t40576) + t40438 * t562 * t568 + F::new(4.0) * t12237 * t1372 * t568 + F::new(6.0) * t3752 * t3879 * t568 + F::new(24.0) * t1375 * t40591 * t40592 - F::new(24.0) * t3882 * t12023 - F::new(4.0) * t3882 * t12438 + F::new(4.0) * t1323 * t12434 * t568;
    t40603
}
