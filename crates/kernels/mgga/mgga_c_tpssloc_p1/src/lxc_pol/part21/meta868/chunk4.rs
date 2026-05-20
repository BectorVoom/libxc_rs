//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3180/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3180<F: Float>(t1227: F, t248: F, t45046: F, t5971: F, t15643: F, t5005: F, t15438: F, t15453: F, t15527: F, t15555: F, t15637: F, t15737: F, t19080: F, t3496: F, t44886: F, t44890: F, t44894: F, t4582: F, t5002: F, t52776: F, t52781: F, t52792: F, t52795: F, t52801: F, t62044: F) -> F {
    let t65935 = t1227 * t248 * t45046 * t5971;
    let t65952 = t5005 * t15643;
    let t65954 = -t52776 / F::new(72.0) + t5002 * t15527 / F::new(1536.0) - t19080 * t3496 / F::new(288.0) - F::new(5.0) / F::new(62208.0) * t65935 - t52781 / F::new(2304.0) - t44886 / F::new(13824.0) - t44890 / F::new(6912.0) + t44894 / F::new(13824.0) + F::new(5.0) / F::new(5184.0) * t52792 - t52795 / F::new(2304.0) - F::new(5.0) / F::new(5184.0) * t1227 * t4582 * t15453 * t62044 + t15737 * t15555 / F::new(384.0) - t15438 * t15637 / F::new(768.0) + t52801 / F::new(2304.0) - t65952 / F::new(864.0);
    t65954
}
