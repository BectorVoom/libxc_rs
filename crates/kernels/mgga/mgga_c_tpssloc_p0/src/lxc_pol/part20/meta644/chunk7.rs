//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2365/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2365<F: Float>(t10214: F, t10263: F, t10390: F, t10877: F, t14130: F, t14167: F, t1539: F, t2979: F, t3048: F, t3071: F, t42380: F, t42403: F, t42412: F, t43361: F, t4562: F, t4565: F, t47689: F, t47693: F, t47720: F, t47742: F, t47767: F, t973: F, t977: F) -> F {
    let t48543 = -t3048 * t14167 / F::new(48.0) - F::new(11.0) / F::new(54.0) * t10263 * t4562 + F::new(11.0) / F::new(81.0) * t10263 * t4565 + t42380 / F::new(1152.0) - t43361 * t3071 * t1539 * t10877 / F::new(768.0) - t42403 / F::new(1152.0) + t42412 / F::new(2304.0) - t10390 * t14130 / F::new(768.0) - t973 * t977 * t47767 / F::new(144.0) - F::new(7.0) / F::new(54.0) * t973 * t10214 * t47742 + t973 * t2979 * t47689 / F::new(72.0) + t973 * t2979 * t47693 / F::new(72.0) + F::new(7.0) / F::new(216.0) * t973 * t10214 * t47720;
    t48543
}
