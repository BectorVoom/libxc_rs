//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1385/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1385<F: Float>(t76995: F, t77017: F, t77151: F, t77483: F, t1020: F, t1021: F, t1041: F, t1044: F, t1618: F, t17607: F, t21580: F, t248: F, t3062: F, t3131: F, t360: F, t369: F, t378: F, t42347: F, t43317: F, t4644: F, t5880: F, t5900: F, t61739: F, t68: F, t70148: F, t70162: F, t70166: F, t70199: F, t70209: F, t70214: F, t70227: F, t75836: F, t76597: F, t76612: F, t76620: F, t76740: F, t76977: F, t973: F, t974: F) -> (F, F) {
    let t77485 = t76995 + t77017 + t77151 + t77483;
    let t77498 = -t70162 / F::new(192.0) + t70166 / F::new(288.0) - F::new(5.0) / F::new(576.0) * t4644 * t21580 + t70148 * t1618 / F::new(768.0) - t17607 * t5900 / F::new(384.0) + t70199 / F::new(1728.0) - t1041 * t248 * t1044 * t76612 / F::new(192.0) + t70209 / F::new(192.0) + t70214 / F::new(384.0) + t76977 * t68 * t369 * t378 / F::new(3072.0) - t1041 * t248 * t1044 * t76620 / F::new(768.0) + F::new(5.0) / F::new(4608.0) * t1041 * t248 * t3062 * t76597 + t70227 / F::new(192.0) + t973 * t974 * t43317 * t75836 / F::new(6.0) + t1020 * t248 * t1021 * t77485 * t360 / F::new(3072.0) + F::new(7.0) / F::new(1536.0) * t42347 * t248 * t1021 * t76740 * t3131 - t61739 * t5880 / F::new(512.0);
    (t77485, t77498)
}
