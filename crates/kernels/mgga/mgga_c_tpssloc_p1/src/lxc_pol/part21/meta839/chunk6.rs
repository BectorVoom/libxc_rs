//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3007/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3007<F: Float>(t5893: F, t698: F, t973: F, t17615: F, t2960: F, t3131: F, t5866: F, t1022: F, t5872: F, t10263: F, t10403: F, t10413: F, t13995: F, t14213: F, t14215: F, t14220: F, t14228: F, t14230: F, t3070: F, t3071: F, t42483: F, t43352: F, t43354: F, t4342: F, t4575: F, t49929: F, t50324: F, t50425: F, t50429: F, t5677: F, t5894: F, t61775: F) -> F {
    let t62832 = t973 * t698 * t5893;
    let t62836 = t2960 * t17615;
    let t62840 = t5866 * t3131;
    let t62845 = t5866 * t1022;
    let t62850 = t5872 * t1022;
    let t62871 = -t62832 / F::new(972.0) + F::new(11.0) / F::new(243.0) * t10263 * t5894 - t62836 / F::new(162.0) - t13995 * t14230 / F::new(576.0) + t10403 * t3071 * t62840 * t14213 / F::new(1152.0) - t10413 * t3071 * t62845 * t14220 / F::new(2304.0) + t42483 * t3071 * t62850 * t14220 / F::new(2304.0) + t3070 * t3071 * t5677 * t14228 / F::new(384.0) - t3070 * t3071 * t4342 * t61775 / F::new(576.0) - t43352 / F::new(13824.0) - F::new(19.0) / F::new(7776.0) * t43354 + t50324 * t4575 / F::new(1152.0) + F::new(5.0) / F::new(1944.0) * t50425 + t50429 / F::new(3456.0) + t49929 * t14215 / F::new(576.0);
    t62871
}
